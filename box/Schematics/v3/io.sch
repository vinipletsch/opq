EESchema Schematic File Version 2
LIBS:opqbox3
LIBS:device
LIBS:power
LIBS:conn
LIBS:opqbox3-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 5 5
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
Text Notes 1300 1700 0    60   ~ 0
JTAG Header
$Comp
L Conn_02x04_Odd_Even J4
U 1 1 5A852B85
P 1550 1200
F 0 "J4" H 1600 1400 50  0000 C CNN
F 1 "Conn_02x04_Odd_Even" H 1600 900 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_2x04_Pitch2.54mm" H 1550 1200 50  0001 C CNN
F 3 "" H 1550 1200 50  0001 C CNN
	1    1550 1200
	1    0    0    -1  
$EndComp
Text HLabel 1850 1200 2    60   BiDi ~ 0
JTAG_JRST
Text HLabel 1350 1300 0    60   BiDi ~ 0
JTAG_JTCK
Text HLabel 1850 1300 2    60   BiDi ~ 0
JTAG_JTDI
Text HLabel 1350 1400 0    60   BiDi ~ 0
JTAG_nRST
Text HLabel 1850 1400 2    60   BiDi ~ 0
JTAG_JTDO
Text HLabel 1350 1200 0    60   BiDi ~ 0
JTAG_JTMS
NoConn ~ 1350 1100
$Comp
L GND #PWR32
U 1 1 5A852BF5
P 2050 950
F 0 "#PWR32" H 2050 700 50  0001 C CNN
F 1 "GND" H 2050 800 50  0000 C CNN
F 2 "" H 2050 950 50  0001 C CNN
F 3 "" H 2050 950 50  0001 C CNN
	1    2050 950 
	1    0    0    -1  
$EndComp
Wire Wire Line
	1850 1100 1900 1100
Wire Wire Line
	1900 1100 1900 900 
Wire Wire Line
	1900 900  2050 900 
Wire Wire Line
	2050 900  2050 950 
$Comp
L Conn_01x04 J5
U 1 1 5A857B11
P 1950 2400
F 0 "J5" H 1950 2600 50  0000 C CNN
F 1 "Conn_01x04" H 1950 2100 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x04_Pitch2.54mm" H 1950 2400 50  0001 C CNN
F 3 "" H 1950 2400 50  0001 C CNN
	1    1950 2400
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR31
U 1 1 5A857B5F
P 1650 2700
F 0 "#PWR31" H 1650 2450 50  0001 C CNN
F 1 "GND" H 1650 2550 50  0000 C CNN
F 2 "" H 1650 2700 50  0001 C CNN
F 3 "" H 1650 2700 50  0001 C CNN
	1    1650 2700
	1    0    0    -1  
$EndComp
Wire Wire Line
	1650 2500 1650 2700
Wire Wire Line
	1750 2600 1650 2600
Wire Wire Line
	1650 2500 1750 2500
Connection ~ 1650 2600
$Comp
L +5V #PWR30
U 1 1 5A857BAF
P 1650 2250
F 0 "#PWR30" H 1650 2100 50  0001 C CNN
F 1 "+5V" H 1650 2390 50  0000 C CNN
F 2 "" H 1650 2250 50  0001 C CNN
F 3 "" H 1650 2250 50  0001 C CNN
	1    1650 2250
	1    0    0    -1  
$EndComp
Wire Wire Line
	1650 2250 1650 2300
Wire Wire Line
	1650 2300 1750 2300
Text HLabel 1750 2400 0    60   BiDi ~ 0
PPS
Text Notes 1400 3050 0    60   ~ 0
4 Pos Header
$Comp
L LED D1
U 1 1 5A85F2BE
P 3850 1150
F 0 "D1" H 3850 1250 50  0000 C CNN
F 1 "LED" H 3850 1050 50  0000 C CNN
F 2 "LEDs:LED_0603_HandSoldering" H 3850 1150 50  0001 C CNN
F 3 "" H 3850 1150 50  0001 C CNN
	1    3850 1150
	-1   0    0    1   
$EndComp
$Comp
L LED D2
U 1 1 5A85F345
P 3850 1450
F 0 "D2" H 3850 1550 50  0000 C CNN
F 1 "LED" H 3850 1350 50  0000 C CNN
F 2 "LEDs:LED_0603_HandSoldering" H 3850 1450 50  0001 C CNN
F 3 "" H 3850 1450 50  0001 C CNN
	1    3850 1450
	-1   0    0    1   
$EndComp
$Comp
L LED D3
U 1 1 5A85F3E5
P 3850 1750
F 0 "D3" H 3850 1850 50  0000 C CNN
F 1 "LED" H 3850 1650 50  0000 C CNN
F 2 "LEDs:LED_0603_HandSoldering" H 3850 1750 50  0001 C CNN
F 3 "" H 3850 1750 50  0001 C CNN
	1    3850 1750
	-1   0    0    1   
$EndComp
$Comp
L R R11
U 1 1 5A85F458
P 3500 1150
F 0 "R11" V 3580 1150 50  0000 C CNN
F 1 "68" V 3500 1150 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3430 1150 50  0001 C CNN
F 3 "" H 3500 1150 50  0001 C CNN
	1    3500 1150
	0    1    1    0   
$EndComp
$Comp
L R R12
U 1 1 5A85F48B
P 3500 1450
F 0 "R12" V 3580 1450 50  0000 C CNN
F 1 "68" V 3500 1450 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3430 1450 50  0001 C CNN
F 3 "" H 3500 1450 50  0001 C CNN
	1    3500 1450
	0    1    1    0   
$EndComp
$Comp
L R R13
U 1 1 5A85F4B8
P 3500 1750
F 0 "R13" V 3580 1750 50  0000 C CNN
F 1 "68" V 3500 1750 50  0000 C CNN
F 2 "Resistors_SMD:R_0402" V 3430 1750 50  0001 C CNN
F 3 "" H 3500 1750 50  0001 C CNN
	1    3500 1750
	0    1    1    0   
$EndComp
$Comp
L GND #PWR34
U 1 1 5A85F4E9
P 4100 1850
F 0 "#PWR34" H 4100 1600 50  0001 C CNN
F 1 "GND" H 4100 1700 50  0000 C CNN
F 2 "" H 4100 1850 50  0001 C CNN
F 3 "" H 4100 1850 50  0001 C CNN
	1    4100 1850
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 1150 4100 1850
Wire Wire Line
	4100 1750 4000 1750
Wire Wire Line
	4100 1450 4000 1450
Connection ~ 4100 1750
Wire Wire Line
	4000 1150 4100 1150
Connection ~ 4100 1450
Text HLabel 3300 1450 0    60   Input ~ 0
LED1
Text HLabel 3300 1750 0    60   Input ~ 0
LED2
Wire Wire Line
	3650 1150 3700 1150
Wire Wire Line
	3650 1450 3700 1450
Wire Wire Line
	3650 1750 3700 1750
Wire Wire Line
	3300 1750 3350 1750
Wire Wire Line
	3300 1450 3350 1450
$Comp
L +5V #PWR33
U 1 1 5A85F8C1
P 3250 1100
F 0 "#PWR33" H 3250 950 50  0001 C CNN
F 1 "+5V" H 3250 1240 50  0000 C CNN
F 2 "" H 3250 1100 50  0001 C CNN
F 3 "" H 3250 1100 50  0001 C CNN
	1    3250 1100
	1    0    0    -1  
$EndComp
Wire Wire Line
	3250 1100 3250 1150
Wire Wire Line
	3250 1150 3350 1150
$EndSCHEMATC
