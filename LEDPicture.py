import time
import serial
import cv2
import os
import threading

files = ["Output", "Output/0", "Output/90", "Output/180", "Output/270"]

for file in files:
    if not os.path.exists(file):
            os.makedirs(file)

cam_port = -1
cam = cv2.VideoCapture(cam_port)

class TakeCameraLatestPictureThread(threading.Thread):
    def __init__(self, camera):
        self.camera = camera
        self.frame = None
        self.ret = False
        super().__init__()
        # Start thread
        self.canRun = True
        self.start()
        self.current_id = 0

    def run(self):
        while self.canRun:
            self.ret, self.frame = self.camera.read()
            self.current_id += 1

latest_picture = TakeCameraLatestPictureThread(cam)

ser = serial.Serial("/dev/ttyUSB0")
ser.baudrate=115200

currentRotation = 0
currentImage = 0

MSG_RESET = bytes("R", "UTF-8")
MSG_NEXT = bytes("N", "UTF-8")

while True:
    count = 0
    print("NEXT")
    ser.write(MSG_NEXT)

    msg = ser.read(1)
    print(msg)
    output = msg.decode("UTF-8")
    print(f"OUTPUT: {output.strip()}")
    try:
        code = output[0]
        print(f"CODE: {code}")
    except Exception:
        continue

    if code == 'S':
        current_count = latest_picture.current_id
        diff = latest_picture.current_id - current_count
        while (diff < 3): ## Ensure 3 pictures taken before we save it
            diff = latest_picture.current_id - current_count
            time.sleep(0.01)

        cv2.imwrite(f"Output/{currentRotation}/image{currentImage}.png", latest_picture.frame)

        print(f"Finished {currentRotation}/image{currentImage}.png")
        currentImage += 1
        time.sleep(0.2)
    elif code == 'F':
        ser.write(MSG_RESET)
        ser.read(1)
        print("Please rotate the tree")
        input("PRESS ENTER WHEN DONE")
        currentRotation += 90
        currentImage = 0

    if (currentRotation == 360):
        break

latest_picture.canRun = False
