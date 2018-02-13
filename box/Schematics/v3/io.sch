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
Sheet 3 4
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
L Conn_02x20_Odd_Even J3
U 1 1 5A8522AA
P 2200 2350
F 0 "J3" H 2250 3350 50  0000 C CNN
F 1 "Conn_02x20_Odd_Even" H 2250 1250 50  0000 C CNN
F 2 "" H 2200 2350 50  0001 C CNN
F 3 "" H 2200 2350 50  0001 C CNN
	1    2200 2350
	1    0    0    -1  
$EndComp
Text HLabel 2000 2350 0    60   BiDi ~ 0
SPI_MISO
Text HLabel 2000 2450 0    60   BiDi ~ 0
SPI_MOSI
Text HLabel 2500 2550 2    60   BiDi ~ 0
SPI_nCS
Text HLabel 2000 2550 0    60   BiDi ~ 0
SPI_CLK
Text HLabel 2500 2250 2    60   BiDi ~ 0
JTAG_nRST
Text HLabel 2500 2150 2    60   Output ~ 0
BOOT_MODE
Text HLabel 2500 1750 2    60   Output ~ 0
UART_RX
Text HLabel 2500 1850 2    60   Input ~ 0
UART_TX
$Comp
L +5V #PWR024
U 1 1 5A852361
P 2550 1400
F 0 "#PWR024" H 2550 1250 50  0001 C CNN
F 1 "+5V" H 2550 1540 50  0000 C CNN
F 2 "" H 2550 1400 50  0001 C CNN
F 3 "" H 2550 1400 50  0001 C CNN
	1    2550 1400
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR025
U 1 1 5A852384
P 3050 1900
F 0 "#PWR025" H 3050 1650 50  0001 C CNN
F 1 "GND" H 3050 1750 50  0000 C CNN
F 2 "" H 3050 1900 50  0001 C CNN
F 3 "" H 3050 1900 50  0001 C CNN
	1    3050 1900
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR026
U 1 1 5A852469
P 1850 2700
F 0 "#PWR026" H 1850 2450 50  0001 C CNN
F 1 "GND" H 1850 2550 50  0000 C CNN
F 2 "" H 1850 2700 50  0001 C CNN
F 3 "" H 1850 2700 50  0001 C CNN
	1    1850 2700
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR027
U 1 1 5A85249C
P 1850 1900
F 0 "#PWR027" H 1850 1650 50  0001 C CNN
F 1 "GND" H 1850 1750 50  0000 C CNN
F 2 "" H 1850 1900 50  0001 C CNN
F 3 "" H 1850 1900 50  0001 C CNN
	1    1850 1900
	1    0    0    -1  
$EndComp
Text HLabel 2500 1950 2    60   BiDi ~ 0
FLAG1
NoConn ~ 2000 1450
NoConn ~ 2000 1550
NoConn ~ 2000 1650
NoConn ~ 2000 1750
NoConn ~ 2000 1950
NoConn ~ 2000 2050
NoConn ~ 2000 2150
NoConn ~ 2000 2250
NoConn ~ 2000 2750
NoConn ~ 2000 2850
NoConn ~ 2000 2950
NoConn ~ 2000 3050
NoConn ~ 2000 3150
NoConn ~ 2000 3250
NoConn ~ 2000 3350
NoConn ~ 2500 3350
NoConn ~ 2500 3250
NoConn ~ 2500 3150
NoConn ~ 2500 3050
NoConn ~ 2500 2950
NoConn ~ 2500 2850
NoConn ~ 2500 2750
NoConn ~ 2500 2650
NoConn ~ 2500 2450
NoConn ~ 2500 2350
NoConn ~ 2500 2050
Text Notes 2000 3700 0    60   ~ 0
RPi Header
Text Notes 4050 2050 0    60   ~ 0
JTAG Header
$Comp
L Conn_02x04_Odd_Even J4
U 1 1 5A852B85
P 4300 1550
F 0 "J4" H 4350 1750 50  0000 C CNN
F 1 "Conn_02x04_Odd_Even" H 4350 1250 50  0000 C CNN
F 2 "" H 4300 1550 50  0001 C CNN
F 3 "" H 4300 1550 50  0001 C CNN
	1    4300 1550
	1    0    0    -1  
$EndComp
Text HLabel 4600 1550 2    60   BiDi ~ 0
JTAG_JRST
Text HLabel 4100 1650 0    60   BiDi ~ 0
JTAG_JTCK
Text HLabel 4600 1650 2    60   BiDi ~ 0
JTAG_JTDI
Text HLabel 4100 1750 0    60   BiDi ~ 0
JTAG_nRST
Text HLabel 4600 1750 2    60   BiDi ~ 0
JTAG_JTDO
Text HLabel 4100 1550 0    60   BiDi ~ 0
JTAG_JTMS
NoConn ~ 4100 1450
$Comp
L GND #PWR028
U 1 1 5A852BF5
P 4800 1300
F 0 "#PWR028" H 4800 1050 50  0001 C CNN
F 1 "GND" H 4800 1150 50  0000 C CNN
F 2 "" H 4800 1300 50  0001 C CNN
F 3 "" H 4800 1300 50  0001 C CNN
	1    4800 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3050 1900 3050 1650
Wire Wire Line
	3050 1650 2500 1650
Wire Wire Line
	1850 2650 2000 2650
Wire Wire Line
	1850 1850 2000 1850
Wire Wire Line
	2500 1450 2550 1450
Wire Wire Line
	2550 1400 2550 1550
Wire Wire Line
	2550 1550 2500 1550
Connection ~ 2550 1450
Wire Wire Line
	1850 1900 1850 1850
Wire Wire Line
	1850 2700 1850 2650
Wire Wire Line
	4600 1450 4650 1450
Wire Wire Line
	4650 1450 4650 1250
Wire Wire Line
	4650 1250 4800 1250
Wire Wire Line
	4800 1250 4800 1300
$Comp
L Conn_01x04 J5
U 1 1 5A857B11
P 4700 2750
F 0 "J5" H 4700 2950 50  0000 C CNN
F 1 "Conn_01x04" H 4700 2450 50  0000 C CNN
F 2 "" H 4700 2750 50  0001 C CNN
F 3 "" H 4700 2750 50  0001 C CNN
	1    4700 2750
	1    0    0    -1  
$EndComp
$Comp
L GND #PWR029
U 1 1 5A857B5F
P 4400 3050
F 0 "#PWR029" H 4400 2800 50  0001 C CNN
F 1 "GND" H 4400 2900 50  0000 C CNN
F 2 "" H 4400 3050 50  0001 C CNN
F 3 "" H 4400 3050 50  0001 C CNN
	1    4400 3050
	1    0    0    -1  
$EndComp
Wire Wire Line
	4400 2850 4400 3050
Wire Wire Line
	4500 2950 4400 2950
Wire Wire Line
	4400 2850 4500 2850
Connection ~ 4400 2950
$Comp
L +5V #PWR030
U 1 1 5A857BAF
P 4400 2600
F 0 "#PWR030" H 4400 2450 50  0001 C CNN
F 1 "+5V" H 4400 2740 50  0000 C CNN
F 2 "" H 4400 2600 50  0001 C CNN
F 3 "" H 4400 2600 50  0001 C CNN
	1    4400 2600
	1    0    0    -1  
$EndComp
Wire Wire Line
	4400 2600 4400 2650
Wire Wire Line
	4400 2650 4500 2650
Text HLabel 4500 2750 0    60   BiDi ~ 0
PPS
Text Notes 4150 3400 0    60   ~ 0
4 Pos Header
$Comp
L LED D1
U 1 1 5A85F2BE
P 6600 1500
F 0 "D1" H 6600 1600 50  0000 C CNN
F 1 "LED" H 6600 1400 50  0000 C CNN
F 2 "" H 6600 1500 50  0001 C CNN
F 3 "" H 6600 1500 50  0001 C CNN
	1    6600 1500
	-1   0    0    1   
$EndComp
$Comp
L LED D2
U 1 1 5A85F345
P 6600 1800
F 0 "D2" H 6600 1900 50  0000 C CNN
F 1 "LED" H 6600 1700 50  0000 C CNN
F 2 "" H 6600 1800 50  0001 C CNN
F 3 "" H 6600 1800 50  0001 C CNN
	1    6600 1800
	-1   0    0    1   
$EndComp
$Comp
L LED D3
U 1 1 5A85F3E5
P 6600 2100
F 0 "D3" H 6600 2200 50  0000 C CNN
F 1 "LED" H 6600 2000 50  0000 C CNN
F 2 "" H 6600 2100 50  0001 C CNN
F 3 "" H 6600 2100 50  0001 C CNN
	1    6600 2100
	-1   0    0    1   
$EndComp
$Comp
L R R11
U 1 1 5A85F458
P 6250 1500
F 0 "R11" V 6330 1500 50  0000 C CNN
F 1 "R" V 6250 1500 50  0000 C CNN
F 2 "" V 6180 1500 50  0001 C CNN
F 3 "" H 6250 1500 50  0001 C CNN
	1    6250 1500
	0    1    1    0   
$EndComp
$Comp
L R R12
U 1 1 5A85F48B
P 6250 1800
F 0 "R12" V 6330 1800 50  0000 C CNN
F 1 "R" V 6250 1800 50  0000 C CNN
F 2 "" V 6180 1800 50  0001 C CNN
F 3 "" H 6250 1800 50  0001 C CNN
	1    6250 1800
	0    1    1    0   
$EndComp
$Comp
L R R13
U 1 1 5A85F4B8
P 6250 2100
F 0 "R13" V 6330 2100 50  0000 C CNN
F 1 "R" V 6250 2100 50  0000 C CNN
F 2 "" V 6180 2100 50  0001 C CNN
F 3 "" H 6250 2100 50  0001 C CNN
	1    6250 2100
	0    1    1    0   
$EndComp
$Comp
L GND #PWR031
U 1 1 5A85F4E9
P 6850 2200
F 0 "#PWR031" H 6850 1950 50  0001 C CNN
F 1 "GND" H 6850 2050 50  0000 C CNN
F 2 "" H 6850 2200 50  0001 C CNN
F 3 "" H 6850 2200 50  0001 C CNN
	1    6850 2200
	1    0    0    -1  
$EndComp
Wire Wire Line
	6850 1500 6850 2200
Wire Wire Line
	6850 2100 6750 2100
Wire Wire Line
	6850 1800 6750 1800
Connection ~ 6850 2100
Wire Wire Line
	6750 1500 6850 1500
Connection ~ 6850 1800
Text HLabel 6050 1800 0    60   Input ~ 0
LED1
Text HLabel 6050 2100 0    60   Input ~ 0
LED2
Wire Wire Line
	6400 1500 6450 1500
Wire Wire Line
	6400 1800 6450 1800
Wire Wire Line
	6400 2100 6450 2100
Wire Wire Line
	6050 2100 6100 2100
Wire Wire Line
	6050 1800 6100 1800
$Comp
L +5V #PWR032
U 1 1 5A85F8C1
P 6000 1450
F 0 "#PWR032" H 6000 1300 50  0001 C CNN
F 1 "+5V" H 6000 1590 50  0000 C CNN
F 2 "" H 6000 1450 50  0001 C CNN
F 3 "" H 6000 1450 50  0001 C CNN
	1    6000 1450
	1    0    0    -1  
$EndComp
Wire Wire Line
	6000 1450 6000 1500
Wire Wire Line
	6000 1500 6100 1500
$EndSCHEMATC
