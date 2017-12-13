// Get rid of the default __blaze-root div element and use the regular <body> element instead.
import { BlazeLayout } from 'meteor/kadira:blaze-layout';
BlazeLayout.setRoot('body');

// Blaze Views already have an isRendered property associated with each view, however it is not a reactive property.
// By attaching a new _isRendered ReactiveVar to each template, we now have a useful way to reactively detect when
// a template has been rendered.
Template.onCreated(function globalOnCreated() {
  const template = this;
  template._isRendered = new ReactiveVar(false);
});
Template.onRendered(function globalOnRendered(){
  const template = this;
  template._isRendered.set(true);
});