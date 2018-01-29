import { ValidatedMethod } from 'meteor/mdg:validated-method';
import { SimpleSchema } from 'meteor/aldeed:simple-schema';
import Moment from 'moment';
import _ from 'lodash';
import { demapify } from 'es6-mapify';
import { Trends } from './TrendsCollection';

export const totalTrendsCount = new ValidatedMethod({
  name: 'Trends.totalTrendsCount',
  validate: new SimpleSchema().validator({ clean: true }),
  run() {
    return Trends.find({}).count();
  },
});

export const monthlyBoxUptime = new ValidatedMethod({
  name: 'Trends.monthlyBoxUptime',
  validate: new SimpleSchema({
    box_id: { type: String },
    month: { type: Number }, // Moment expects 0-indexed values for months.
    year: { type: Number },
  }).validator({ clean: true }),
  run({ box_id, month, year }) {
    const targetMoment = Moment().month(month).year(year);
    const startOfMonthMillis = Moment(targetMoment).startOf('month').valueOf();
    const endOfMonthMillis = Moment(targetMoment).endOf('month').valueOf();
    const daysInMonth = Moment(targetMoment).daysInMonth();

    // Expected interval between trend docs should be 1 minute.
    // This means we expect 60 docs an hour per box, which is 1440 docs a day, and (1440 * 28 to 31) a month.
    const expectedNumMonthlyDocs = 60 * 24 * daysInMonth;

    // Get all Trend docs for the given OpqBox
    const trends = Trends.find({
      box_id,
      timestamp_ms: { $gte: startOfMonthMillis, $lte: endOfMonthMillis },
    });

    const numMissingDocs = expectedNumMonthlyDocs - trends.count();
    const downtime = numMissingDocs / expectedNumMonthlyDocs;
    const uptime = (1 - downtime) * 100;
    return uptime;
  },
});

export const monthlyBoxTrends = new ValidatedMethod({
  name: 'Trends.monthlyBoxTrends',
  validate: new SimpleSchema({
    box_id: { type: String },
    month: { type: Number }, // Moment expects 0-indexed values for months.
    year: { type: Number },
  }).validator({ clean: true }),
  run({ box_id, month, year }) {
    const targetMoment = Moment().month(month).year(year);
    const startOfMonthMillis = Moment(targetMoment).startOf('month').valueOf();
    const endOfMonthMillis = Moment(targetMoment).endOf('month').valueOf();
    const daysInMonth = Moment(targetMoment).daysInMonth();

    // Get all Trend docs for the given OpqBox
    const monthBoxTrends = Trends.find({
      box_id,
      timestamp_ms: { $gte: startOfMonthMillis, $lte: endOfMonthMillis },
    }).fetch();

    // Daily trend values (DTV)
    // We keep track of individual counts because there is no guarantee that all trend docs will have each of
    // voltage, frequency, and thd values available.
    const dailyTrendValues = {
      voltage: {
        min: Number.MAX_SAFE_INTEGER,
        minDate: null,
        max: Number.MIN_SAFE_INTEGER,
        maxDate: null,
        average: 0,
        count: 0,
      },
      frequency: {
        min: Number.MAX_SAFE_INTEGER,
        minDate: null,
        max: Number.MIN_SAFE_INTEGER,
        maxDate: null,
        average: 0,
        count: 0,
      },
      thd: {
        min: Number.MAX_SAFE_INTEGER,
        minDate: null,
        max: Number.MIN_SAFE_INTEGER,
        maxDate: null,
        average: 0,
        count: 0,
      },
      totalDocCount: 0,
    };

    // Maps the day of month (int) to dailyTrendValues object.
    const trendsMap = new Map();
    // Populate initial key, val pairs.
    for (let i = 1; i < daysInMonth + 1; i++) {
      trendsMap.set(i, _.cloneDeep(dailyTrendValues));
    }

    // Calculate 1 month worth of daily trends. Note: It might seem strange that we are keeping track of min/max date
    // for daily trends, but we can then subsequently use these dates for monthly trends, which is more useful.
    monthBoxTrends.forEach(trend => {
      const dayOfMonthInt = Moment(trend.timestamp_ms).date();
      const dtv = trendsMap.get(dayOfMonthInt); // Daily Trend Values
      dtv.totalDocCount++;

      // Voltage
      if (trend.voltage) {
        dtv.voltage.count++;
        if (trend.voltage.min < dtv.voltage.min) {
          dtv.voltage.min = trend.voltage.min;
          dtv.voltage.minDate = trend.timestamp_ms;
        }
        if (trend.voltage.max > dtv.voltage.max) {
          dtv.voltage.max = trend.voltage.max;
          dtv.voltage.maxDate = trend.timestamp_ms;
        }
        dtv.voltage.average += (trend.voltage.average - dtv.voltage.average) / dtv.voltage.count;
      }

      // Frequency
      if (trend.frequency) {
        dtv.frequency.count++;
        if (trend.frequency.min < dtv.frequency.min) {
          dtv.frequency.min = trend.frequency.min;
          dtv.frequency.minDate = trend.timestamp_ms;
        }
        if (trend.frequency.max > dtv.frequency.max) {
          dtv.frequency.max = trend.frequency.max;
          dtv.frequency.maxDate = trend.timestamp_ms;
        }
        dtv.frequency.average += (trend.frequency.average - dtv.frequency.average) / dtv.frequency.count;
      }

      // THD
      if (trend.thd) {
        dtv.thd.count++;
        if (trend.thd.min < dtv.thd.min) {
          dtv.thd.min = trend.thd.min;
          dtv.thd.minDate = trend.timestamp_ms;
        }
        if (trend.thd.max > dtv.thd.max) {
          dtv.thd.max = trend.thd.max;
          dtv.thd.maxDate = trend.timestamp_ms;
        }
        dtv.thd.average += (trend.thd.average - dtv.thd.average) / dtv.thd.count;
      }
    });

    // Let's remove daily trend fields (voltage, freq, thd) which did not exist (most likely thd values)
    trendsMap.forEach(dtv => {
      if (dtv.voltage.count === 0) delete dtv.voltage; // eslint-disable-line no-param-reassign
      if (dtv.frequency.count === 0) delete dtv.frequency; // eslint-disable-line no-param-reassign
      if (dtv.thd.count === 0) delete dtv.thd; // eslint-disable-line no-param-reassign
    });

    // Now calculate monthly trends, which is much quicker - one iteration per day of month. We only need a single
    // object to hold monthly trends data.
    const mtv = _.cloneDeep(dailyTrendValues); // mtv = Monthly Trend Values
    trendsMap.forEach(dtv => { // dtv = Daily Trend Values
      // Represents the true total doc count of all trend documents parsed for this mont.
      mtv.totalDocCount += dtv.totalDocCount;

      // Voltage
      if (dtv.voltage) {
        mtv.voltage.count++;
        if (dtv.voltage.min < mtv.voltage.min) {
          mtv.voltage.min = dtv.voltage.min;
          mtv.voltage.minDate = dtv.voltage.minDate;
        }
        if (dtv.voltage.max > mtv.voltage.max) {
          mtv.voltage.max = dtv.voltage.max;
          mtv.voltage.maxDate = dtv.voltage.maxDate;
        }
        mtv.voltage.average += (dtv.voltage.average - mtv.voltage.average) / mtv.voltage.count;
      }

      // Frequency
      if (dtv.frequency) {
        mtv.frequency.count++;
        if (dtv.frequency.min < mtv.frequency.min) {
          mtv.frequency.min = dtv.frequency.min;
          mtv.frequency.minDate = dtv.frequency.minDate;
        }
        if (dtv.frequency.max > mtv.frequency.max) {
          mtv.frequency.max = dtv.frequency.max;
          mtv.frequency.maxDate = dtv.frequency.maxDate;
        }
        mtv.frequency.average += (dtv.frequency.average - mtv.frequency.average) / mtv.frequency.count;
      }

      // THD
      if (dtv.thd) {
        mtv.thd.count++;
        if (dtv.thd.min < mtv.thd.min) {
          mtv.thd.min = dtv.thd.min;
          mtv.thd.minDate = dtv.thd.minDate;
        }
        if (dtv.thd.max > mtv.thd.max) {
          mtv.thd.max = dtv.thd.max;
          mtv.thd.maxDate = dtv.thd.maxDate;
        }
        mtv.thd.average += (dtv.thd.average - mtv.thd.average) / mtv.thd.count;
      }
    });

    // Similarly, let's also remove the monthly trend fields which did not exist (voltage, freq, thd), if any.
    if (mtv.voltage.count === 0) delete mtv.voltage; // eslint-disable-line no-param-reassign
    if (mtv.frequency.count === 0) delete mtv.frequency; // eslint-disable-line no-param-reassign
    if (mtv.thd.count === 0) delete mtv.thd; // eslint-disable-line no-param-reassign

    // Daily uptime calculations
    const EXPECTED_DAILY_DOCS = 60 * 24; // We expect 1 trend doc per minute, per device.
    trendsMap.forEach(dtv => {
      const missingDocCount = EXPECTED_DAILY_DOCS - dtv.totalDocCount;
      const downtime = missingDocCount / EXPECTED_DAILY_DOCS;
      dtv.uptime = (1 - downtime) * 100; // eslint-disable-line no-param-reassign
    });

    // Monthly uptime calculations
    const EXPECTED_MONTHLY_DOCS = 60 * 24 * daysInMonth;
    const missingDocCount = EXPECTED_MONTHLY_DOCS - mtv.totalDocCount;
    const downtime = missingDocCount / EXPECTED_MONTHLY_DOCS;
    mtv.uptime = (1 - downtime) * 100;

    return {
      dailyTrends: demapify(trendsMap),
      monthlyTrends: mtv,
    };
  },
});

export const getMostRecentTrendMonth = new ValidatedMethod({
  name: 'Trends.mostRecentTrendMonth',
  validate: new SimpleSchema({}).validator({ clean: true }),
  run() {
    if (!this.isSimulation) {
      const trend = Trends.findOne({}, { sort: { timestamp_ms: -1 } });
      const trendMoment = Moment(trend.timestamp_ms);
      const month = trendMoment.month(); // 0-indexed month integers (January is 0)
      const year = trendMoment.year();
      return { box_id: trend.box_id, month, year };
    }
    return null;
  },
});