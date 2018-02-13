EESchema Schematic File Version 2
LIBS:opqbox3
LIBS:device
LIBS:power
LIBS:conn
LIBS:contrib
LIBS:opqbox3-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L GND #PWR01
U 1 1 5A823328
P 8850 3150
F 0 "#PWR01" H 8850 2900 50  0001 C CNN
F 1 "GND" H 8850 3000 50  0000 C CNN
F 2 "" H 8850 3150 50  0001 C CNN
F 3 "" H 8850 3150 50  0001 C CNN
	1    8850 3150
	1    0    0    -1  
$EndComp
$Comp
L CP C3
U 1 1 5A823341
P 8050 2550
F 0 "C3" H 8075 2650 50  0000 L CNN
F 1 "CP" H 8075 2450 50  0000 L CNN
F 2 "" H 8088 2400 50  0001 C CNN
F 3 "" H 8050 2550 50  0001 C CNN
	1    8050 2550
	1    0    0    -1  
$EndComp
$Comp
L CP C4
U 1 1 5A82336C
P 9650 2550
F 0 "C4" H 9675 2650 50  0000 L CNN
F 1 "CP" H 9675 2450 50  0000 L CNN
F 2 "" H 9688 2400 50  0001 C CNN
F 3 "" H 9650 2550 50  0001 C CNN
	1    9650 2550
	1    0    0    -1  
$EndComp
Wire Wire Line
	9650 2000 9650 2400
Wire Wire Line
	9650 2250 9450 2250
Wire Wire Line
	8050 2700 8050 3050
Wire Wire Line
	8050 3050 9650 3050
Connection ~ 8850 3050
Wire Wire Line
	9650 3050 9650 2700
$Comp
L AP2120N-3.3TRG1 REG1
U 1 1 5A823602
P 8250 2250
F 0 "REG1" H 8450 2500 60  0000 L CNN
F 1 "AP2120N-3.3TRG1" H 8450 2400 60  0000 L CNN
F 2 "" H 8250 2250 60  0001 C CNN
F 3 "" H 8250 2250 60  0001 C CNN
	1    8250 2250
	1    0    0    -1  
$EndComp
$Comp
L +5V #PWR02
U 1 1 5A8236B0
P 8050 2000
F 0 "#PWR02" H 8050 1850 50  0001 C CNN
F 1 "+5V" H 8050 2140 50  0000 C CNN
F 2 "" H 8050 2000 50  0001 C CNN
F 3 "" H 8050 2000 50  0001 C CNN
	1    8050 2000
	1    0    0    -1  
$EndComp
$Comp
L +3.3V #PWR03
U 1 1 5A8236CA
P 9650 2000
F 0 "#PWR03" H 9650 1850 50  0001 C CNN
F 1 "+3.3V" H 9650 2140 50  0000 C CNN
F 2 "" H 9650 2000 50  0001 C CNN
F 3 "" H 9650 2000 50  0001 C CNN
	1    9650 2000
	1    0    0    -1  
$EndComp
Connection ~ 8050 2250
Connection ~ 9650 2250
$Comp
L Fuse F1
U 1 1 5A823CA9
P 3300 2200
F 0 "F1" V 3380 2200 50  0000 C CNN
F 1 "Fuse" V 3225 2200 50  0000 C CNN
F 2 "" V 3230 2200 50  0001 C CNN
F 3 "" H 3300 2200 50  0001 C CNN
	1    3300 2200
	0    1    1    0   
$EndComp
$Comp
L Varistor RV1
U 1 1 5A823D68
P 3600 2450
F 0 "RV1" V 3725 2450 50  0000 C CNN
F 1 "Varistor" V 3475 2450 50  0000 C CNN
F 2 "" V 3530 2450 50  0001 C CNN
F 3 "" H 3600 2450 50  0001 C CNN
	1    3600 2450
	1    0    0    -1  
$EndComp
Wire Wire Line
	3600 1850 3600 2300
Wire Wire Line
	2700 2750 4100 2750
Wire Wire Line
	3600 2750 3600 2600
Wire Wire Line
	4100 2200 4100 2450
Connection ~ 3600 2200
Wire Wire Line
	4100 2750 4100 2550
Connection ~ 3600 2750
$Comp
L IRM-05-5 PSU1
U 1 1 5A82394D
P 4100 2450
F 0 "PSU1" H 4300 2700 60  0000 L CNN
F 1 "IRM-05-5" H 4300 2600 60  0000 L CNN
F 2 "" H 4100 2450 60  0001 C CNN
F 3 "" H 4100 2450 60  0001 C CNN
	1    4100 2450
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR04
U 1 1 5A823A0C
P 5250 2750
F 0 "#PWR04" H 5250 2500 50  0001 C CNN
F 1 "GND" H 5250 2600 50  0000 C CNN
F 2 "" H 5250 2750 50  0001 C CNN
F 3 "" H 5250 2750 50  0001 C CNN
	1    5250 2750
	1    0    0    -1  
$EndComp
Wire Wire Line
	5000 2550 6350 2550
Wire Wire Line
	5250 2550 5250 2750
$Comp
L CP C1
U 1 1 5A823A87
P 5550 2350
F 0 "C1" H 5575 2450 50  0000 L CNN
F 1 "CP" H 5575 2250 50  0000 L CNN
F 2 "" H 5588 2200 50  0001 C CNN
F 3 "" H 5550 2350 50  0001 C CNN
	1    5550 2350
	1    0    0    -1  
$EndComp
$Comp
L CP C2
U 1 1 5A823ADE
P 6350 2350
F 0 "C2" H 6375 2450 50  0000 L CNN
F 1 "CP" H 6375 2250 50  0000 L CNN
F 2 "" H 6388 2200 50  0001 C CNN
F 3 "" H 6350 2350 50  0001 C CNN
	1    6350 2350
	1    0    0    -1  
$EndComp
$Comp
L L L1
U 1 1 5A823B3B
P 5950 2150
F 0 "L1" V 5900 2150 50  0000 C CNN
F 1 "L" V 6025 2150 50  0000 C CNN
F 2 "" H 5950 2150 50  0001 C CNN
F 3 "" H 5950 2150 50  0001 C CNN
	1    5950 2150
	0    -1   -1   0   
$EndComp
Wire Wire Line
	6100 2150 6650 2150
Connection ~ 5250 2550
Connection ~ 5550 2550
Connection ~ 5550 2150
Wire Wire Line
	5550 2200 5550 2150
$Comp
L +5V #PWR05
U 1 1 5A823E92
P 6350 2000
F 0 "#PWR05" H 6350 1850 50  0001 C CNN
F 1 "+5V" H 6350 2140 50  0000 C CNN
F 2 "" H 6350 2000 50  0001 C CNN
F 3 "" H 6350 2000 50  0001 C CNN
	1    6350 2000
	1    0    0    -1  
$EndComp
Connection ~ 6350 2150
Text GLabel 3400 1850 0    60   Input ~ 0
L_MEASURE
Wire Wire Line
	3400 1850 3600 1850
$Comp
L NEUT #PWR06
U 1 1 5A824199
P 3050 2800
F 0 "#PWR06" H 3050 2650 50  0001 C CNN
F 1 "NEUT" H 3050 2950 50  0000 C CNN
F 2 "" H 3050 2800 50  0001 C CNN
F 3 "" H 3050 2800 50  0001 C CNN
	1    3050 2800
	-1   0    0    1   
$EndComp
Connection ~ 3050 2750
$Comp
L LINE #PWR07
U 1 1 5A82420A
P 3050 2250
F 0 "#PWR07" H 3050 2100 50  0001 C CNN
F 1 "LINE" H 3050 2400 50  0000 C CNN
F 2 "" H 3050 2250 50  0001 C CNN
F 3 "" H 3050 2250 50  0001 C CNN
	1    3050 2250
	-1   0    0    1   
$EndComp
Connection ~ 3050 2200
$Comp
L Conn_01x02_Male J1
U 1 1 5A8248C2
P 2500 2100
F 0 "J1" H 2500 2200 50  0000 C CNN
F 1 "Conn_01x02_Male" H 2500 1900 50  0000 C CNN
F 2 "" H 2500 2100 50  0001 C CNN
F 3 "" H 2500 2100 50  0001 C CNN
	1    2500 2100
	1    0    0    -1  
$EndComp
$Comp
L Conn_01x02_Male J2
U 1 1 5A82497D
P 2500 2650
F 0 "J2" H 2500 2750 50  0000 C CNN
F 1 "Conn_01x02_Male" H 2500 2450 50  0000 C CNN
F 2 "" H 2500 2650 50  0001 C CNN
F 3 "" H 2500 2650 50  0001 C CNN
	1    2500 2650
	1    0    0    -1  
$EndComp
Wire Wire Line
	2700 2100 2750 2100
Wire Wire Line
	2750 2100 2750 2200
Connection ~ 2750 2200
Wire Wire Line
	2700 2650 2750 2650
Wire Wire Line
	2750 2650 2750 2750
Connection ~ 2750 2750
Wire Wire Line
	5550 2550 5550 2500
Wire Wire Line
	6350 2550 6350 2500
Wire Wire Line
	8250 2250 8050 2250
Wire Wire Line
	8850 2850 8850 3150
Wire Wire Line
	6350 2000 6350 2200
Wire Wire Line
	8050 2000 8050 2400
$Comp
L PWR_FLAG #FLG08
U 1 1 5A825B85
P 3950 2800
F 0 "#FLG08" H 3950 2875 50  0001 C CNN
F 1 "PWR_FLAG" H 3950 2950 50  0000 C CNN
F 2 "" H 3950 2800 50  0001 C CNN
F 3 "" H 3950 2800 50  0001 C CNN
	1    3950 2800
	-1   0    0    1   
$EndComp
$Comp
L PWR_FLAG #FLG09
U 1 1 5A825D4F
P 3900 2150
F 0 "#FLG09" H 3900 2225 50  0001 C CNN
F 1 "PWR_FLAG" H 3900 2300 50  0000 C CNN
F 2 "" H 3900 2150 50  0001 C CNN
F 3 "" H 3900 2150 50  0001 C CNN
	1    3900 2150
	1    0    0    -1  
$EndComp
Wire Wire Line
	2700 2200 3150 2200
Wire Wire Line
	3450 2200 4100 2200
Wire Wire Line
	3050 2150 3050 2250
Wire Wire Line
	3050 2800 3050 2750
Wire Wire Line
	3950 2800 3950 2750
Connection ~ 3950 2750
Wire Wire Line
	3900 2150 3900 2200
Connection ~ 3900 2200
$Comp
L PWR_FLAG #FLG010
U 1 1 5A826550
P 6650 2000
F 0 "#FLG010" H 6650 2075 50  0001 C CNN
F 1 "PWR_FLAG" H 6650 2150 50  0000 C CNN
F 2 "" H 6650 2000 50  0001 C CNN
F 3 "" H 6650 2000 50  0001 C CNN
	1    6650 2000
	1    0    0    -1  
$EndComp
Wire Wire Line
	6650 2150 6650 2000
$Comp
L PWR_FLAG #FLG011
U 1 1 5A82679F
P 3050 2150
F 0 "#FLG011" H 3050 2225 50  0001 C CNN
F 1 "PWR_FLAG" H 3050 2300 50  0000 C CNN
F 2 "" H 3050 2150 50  0001 C CNN
F 3 "" H 3050 2150 50  0001 C CNN
	1    3050 2150
	1    0    0    -1  
$EndComp
$Comp
L PWR_FLAG #FLG012
U 1 1 5A8268AF
P 5950 2750
F 0 "#FLG012" H 5950 2825 50  0001 C CNN
F 1 "PWR_FLAG" H 5950 2900 50  0000 C CNN
F 2 "" H 5950 2750 50  0001 C CNN
F 3 "" H 5950 2750 50  0001 C CNN
	1    5950 2750
	-1   0    0    1   
$EndComp
Wire Wire Line
	5950 2750 5950 2550
Connection ~ 5950 2550
Wire Wire Line
	5000 2450 5100 2450
Wire Wire Line
	5100 2450 5100 2150
Wire Wire Line
	5100 2150 5800 2150
Text GLabel 1450 3500 0    60   Input ~ 0
L_MEASURE
$EndSCHEMATC
