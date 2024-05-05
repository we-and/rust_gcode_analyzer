; Example G-Code for a simple rectangle print
G21 ; Set units to millimeters
G90 ; Use absolute positioning
M82 ; Set extruder to absolute mode
M104 S200 ; Set extruder temperature to 200°C
M140 S60 ; Set bed temperature to 60°C
G28 ; Home all axes
M109 S200 ; Wait for extruder to reach target temperature
M190 S60 ; Wait for bed to reach target temperature
G92 E0 ; Reset extruder distance position

; Begin layer 1
G1 F1500 ; Set speed to 1500mm/min
G1 X2 Y2 Z0.3 F5000 ; Move to start position
G1 X2 Y60 E15 ; Move to Y60 extruding filament
G1 X60 Y60 E30 ; Move to X60 extruding filament
G1 X60 Y2 E45 ; Move to Y2 extruding filament
G1 X2 Y2 E60 ; Move back to start, completing square

; Move up before starting next layer
G1 Z1.0 ; Move Z up 0.7 mm

; Begin layer 2
G1 X2 Y2 Z1.0 F5000 ; Move to start position of second layer
G1 X2 Y60 E75 ; Move to Y60 extruding filament
G1 X60 Y60 E90 ; Move to X60 extruding filament
G1 X60 Y2 E105 ; Move to Y2 extruding filament
G1 X2 Y2 E120 ; Move back to start, completing second square

; End of print
G92 E0 ; Reset extruder distance position
M104 S0 ; Turn off extruder heater
M140 S0 ; Turn off bed heater
G91 ; Set to relative positioning
G1 E-1 F300 ; Retract the filament a bit
G28 X0 ; Home X axis
M84 ; Disable motors