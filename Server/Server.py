import os
import socket
import sys
import time
from os import _exit

import msgpack
import requests

from ServerWorker import ServerWorker
from VideoStream import VideoStream


class Server:
    RTSP_PORT = 1234
    RTP_PORT = 1234
    topologia = {}

    VIDEO_PATH = "../video/"

    SETUP = 0
    PLAY = 1
    # ACK = 2
    TEARDOWN = 3

    INIT = 0
    READY = 1
    PLAYING = 2
    state = INIT

    clientInfo = {}
    active_clients = False

    def read_bootstrapper(self):

        data = requests.get('http://0.0.0.0:8080')
        lines = data.text.split('\n')
        for line in lines[:-1]:
            linha = line.split('-')
            self.topologia[linha[0]] = linha[1].split(',')

        print(self.topologia)
        return self.topologia

    def flood(self, SERVER_ADDRESS, video_file):
        vizinhos = self.topologia[SERVER_ADDRESS]

        payload = {
            "source": SERVER_ADDRESS,
            "jumps": 0,
            "timestamp": int(time.time() * 1000)
        }

        packet = bytearray([])
        packet.append(0)
        pack_data = msgpack.packb(payload, use_bin_type=True)

        size = len(pack_data) + 3
        packet += size.to_bytes(2, 'big')
        packet += pack_data

        for vizinho in vizinhos:
            self.clientInfo[vizinho] = {}
            self.clientInfo[vizinho]['videoStream'] = VideoStream(video_file)
            self.clientInfo[vizinho]['state'] = self.READY
            self.clientInfo[vizinho]['active_clients'] = False
            self.clientInfo[vizinho]['address'] = vizinho
            self.clientInfo[vizinho]['rtpPort'] = self.RTP_PORT
            s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            try:
                s.connect((vizinho, self.RTSP_PORT))
                s.send(packet)
            except:
                print(f"Connection to {vizinho} failed\n")

    def processRtspRequest(self, data, addr):
        """Process RTSP request sent from the client."""
        # Get the request type and size
        requestType = int.from_bytes([data[0]], 'big')
        size = int.from_bytes(data[1:3], 'big')

        print(self.clientInfo)

        # Process SETUP request
        if requestType == self.SETUP:
            if self.clientInfo[addr]['state'] == self.INIT:

                print("processing SETUP\n")

                try:
                    payload = msgpack.unpackb(data[3:size], raw=False)
                    self.clientInfo[addr]['videoStream'] = VideoStream(self.video_file)
                    self.clientInfo[addr]['state'] = self.READY
                except:
                    print("Error in payload or in video Stream\n")

                # Assign fixed RTP port for UDP connection
                self.clientInfo[addr]['rtpPort'] = self.RTP_PORT


        # Process PLAY request
        elif requestType == self.PLAY:
            if self.clientInfo[addr]['state'] == self.READY:
                print("processing PLAY\n")
                self.clientInfo[addr]['active_clients'] = True
                self.clientInfo[addr]['state'] = self.PLAYING

                # Create a new socket for RTP/UDP that will be accessed 
                # by the thread already running at his point
                self.clientInfo[addr]["rtpSocket"] = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)


        # Process TEARDOWN request
        elif requestType == self.TEARDOWN:

            print("processing TEARDOWN\n")
            self.clientInfo[addr]['active_clients'] = False

    def main(self):
        # Default parameters
        server_port = 1234
        video_file = "video/movie.Mjpeg"
        server_address = "10.0.0.10"

        try:
            if len(sys.argv) > 1:
                server_port = int(sys.argv[1])
                video_file = sys.argv[2]
                server_address = sys.argv[3]

        except:
            print("[Usage: Server.py server_port Video_file server_address]")
            return

        self.read_bootstrapper()
        self.flood(server_address, video_file)

        for vizinho in self.topologia[server_address]:
            ServerWorker(self.clientInfo[vizinho], video_file).run()

        rtspSocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        rtspSocket.bind(('', server_port))
        rtspSocket.listen(5)

        while True:
            sock, (addr, _) = rtspSocket.accept()
            data = sock.recv(1500)
            self.processRtspRequest(data, addr)


if __name__ == "__main__":
    try:
        (Server()).main()
    except KeyboardInterrupt:
        print("Server terminated")
        _exit(0)
