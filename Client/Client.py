import os
import socket
import threading
import tkinter.messagebox
from random import randint
from tkinter import *

from PIL import Image, ImageTk

from RtpPacket import RtpPacket

CACHE_FILE_NAME = "cache-"
CACHE_FILE_EXT = ".jpg"


class Client:
    # State
    INIT = 0
    READY = 1
    PLAYING = 2
    state = READY

    # Requests
    FLOODING = 0
    PLAY = 1
    PAUSE = 2
    ACK = 2
    TEARDOWN = 3

    def __init__(self, master, serveraddr, rtpsPort, rtpport):
        # Create a new datagram socket to receive RTP packets from the stream provider
        self.rtpSocket = socket.socket(family=socket.AF_INET, type=socket.SOCK_DGRAM)
        self.rtspSocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.playEvent = threading.Event()
        self.master = master
        self.master.protocol("WM_DELETE_WINDOW", self.exit_window)
        self.createWidgets()
        self.serverAddr = serveraddr
        self.rtpsPort = int(rtpsPort)
        self.rtpPort = int(rtpport)
        self.rtspSeq = 0
        self.sessionId = randint(100000, 999999)
        self.requestSent = -1
        self.teardownRequested = False
        self.connectToServer()
        self.frameNbr = 0

    def createWidgets(self):
        """Build GUI."""

        # Create Play button		
        self.start = Button(self.master, width=20, padx=3, pady=3)
        self.start["text"] = "Play"
        self.start["command"] = self.playMovie
        self.start.grid(row=1, column=1, padx=2, pady=2)

        # Create Pause button			
        self.pause = Button(self.master, width=20, padx=3, pady=3)
        self.pause["text"] = "Pause"
        self.pause["command"] = self.pauseMovie
        self.pause.grid(row=1, column=2, padx=2, pady=2)

        # Create Teardown button
        self.teardown = Button(self.master, width=20, padx=3, pady=3)
        self.teardown["text"] = "Teardown"
        self.teardown["command"] = self.exitClient
        self.teardown.grid(row=1, column=3, padx=2, pady=2)

        # Create a label to display the movie
        self.label = Label(self.master, height=19)
        self.label.grid(row=0, column=0, columnspan=4, sticky=W + E + N + S, padx=5, pady=5)

    def setupMovie(self):
        """Setup button handler."""
        if self.state == self.INIT:
            self.sendRtspRequest(self.FLOODING)

    def exitClient(self):
        """Teardown button handler."""
        try:
            self.sendRtspRequest(self.TEARDOWN)
        except:
            pass

        try:
            self.master.destroy()  # Close the gui window
            os.remove(CACHE_FILE_NAME + str(self.sessionId) + CACHE_FILE_EXT)  # Delete the cache image from video
        except:
            pass

    def pauseMovie(self):
        """Pause button handler."""
        if self.state == self.PLAYING:
            self.sendRtspRequest(self.PAUSE)

    def playMovie(self):
        """Play button handler."""

        if self.state == self.READY:
            # Create a new thread to listen for RTP packets
            self.sendRtspRequest(self.PLAY)
            threading.Thread(target=self.listenRtp).start()
            self.playEvent.clear()

    def listenRtp(self):
        """Listen for RTP packets."""
        while True:
            try:
                data = self.rtpSocket.recv(20480)
                # print(f"Received {len(data)} bytes from rtp socket.")

                if data:
                    rtpPacket = RtpPacket()
                    rtpPacket.decode(data)
                    currFrameNbr = rtpPacket.seqNum()
                    # print(f"Current Seq Num: {currFrameNbr}")

                    if currFrameNbr > self.frameNbr:  # Discard the late packet
                        self.frameNbr = currFrameNbr
                        self.updateMovie(self.writeFrame(rtpPacket.getPayload()))
            except:
                # Stop listening upon requesting PAUSE or TEARDOWN
                if self.playEvent.is_set():
                    break

                # Close the RTP socket if a TEARDOWN was requested
                if self.teardownRequested:
                    self.rtpSocket.shutdown(socket.SHUT_RDWR)
                    self.rtpSocket.close()
                    break

    def writeFrame(self, data):
        """Write the received frame to a temp image file. Return the image file."""
        cache_name = CACHE_FILE_NAME + str(self.sessionId) + CACHE_FILE_EXT
        file = open(cache_name, "wb")
        file.write(data)
        file.close()

        return cache_name

    def updateMovie(self, imageFile):
        """Update the image file as video frame in the GUI."""
        photo = ImageTk.PhotoImage(Image.open(imageFile))
        self.label.configure(image=photo, height=288)
        self.label.image = photo

    def connectToServer(self):
        """Connect to the Server. Start a new RTSP/TCP session."""
        try:
            self.rtspSocket.connect((self.serverAddr, self.rtpsPort))
            print(f"Connected to {self.serverAddr}:{self.rtpsPort}")
        except:
            tkinter.messagebox.showwarning('Connection Failed',
                                           'Connection to \'%s\' failed.' % self.serverAddr)

    def sendRtspRequest(self, requestCode):
        """Send RTSP request to the server."""

        request = bytearray([])

        # Play request
        if requestCode == self.PLAY and self.state == self.READY:
            request = self.play_stream(request)

        # Pause request
        elif requestCode == self.PAUSE and self.state == self.PLAYING:
            request = self.pause_stream(request)

        # Teardown request
        elif requestCode == self.TEARDOWN and not self.state == self.INIT:
            request = self.teardown_stream(request)

        # Send the RTSP request using rtspSocket.
        self.rtspSocket.send(request)

        print(f"Request sent: {requestCode}")

    def teardown_stream(self, request):
        print('\nTearing down the stream\n')

        # Write the RTSP request to be sent.
        request.append(self.TEARDOWN)
        size = 0
        request += size.to_bytes(2, 'big')
        self.playEvent.set()

        # Keep track of the sent request.
        self.requestSent = self.TEARDOWN
        self.teardownRequested = True

        return request

    def pause_stream(self, request):
        print('\nPausing the stream\n')

        # Write the RTSP request to be sent.
        # For the node a teardown and a Pause are the same,
        # Only the client knows the difference
        request.append(self.TEARDOWN)
        size = 0
        request += size.to_bytes(4, 'big')
        self.playEvent.set()

        # Keep track of the sent request.
        self.state == self.READY
        self.requestSent = self.PAUSE
        return request

    def play_stream(self, request):
        print('\nPlaying the stream\n')

        # Write the RTSP request to be sent.
        request.append(self.PLAY)
        size = 3
        request += size.to_bytes(2, 'big')

        # Keep track of the sent request.
        self.requestSent = self.PLAY
        self.state == self.PLAYING
        self.openRtpPort()
        return request

    def openRtpPort(self):
        """Open RTP socket binded to a specified port."""

        try:
            # Bind the socket to the address using the RTP port given by the client user
            self.rtpSocket.bind(("0.0.0.0", self.rtpPort))
            print('\nOpening Rtp Port\n')
        except:
            tkinter.messagebox.showwarning('Unable to Bind', 'Unable to bind PORT=%d' % self.rtpPort)

    def exit_window(self):
        """Handler on explicitly closing the GUI window."""
        self.pauseMovie()

        if tkinter.messagebox.askokcancel("Quit?", "Are you sure you want to quit?"):
            self.exitClient()
        else:  # When the user presses cancel, resume playing.
            self.playMovie()
