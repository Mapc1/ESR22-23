from math import floor
import sys, socket, requests, msgpack, threading
from VideoStream import VideoStream
import time
from ServerWorker import ServerWorker

class Server:

	BOOTSTRAPPER_PORT = 8080
	NODE_PORT = 1234
	topologia = {}
	vizinhos_ip = []

	VIDEO_PATH = "../video/"
	RTP_PORT = 1234

	SETUP = 0
	PLAY = 1
	# PAUSE = 'PAUSE'
	TEARDOWN = 3
	
	INIT = 0
	READY = 1
	PLAYING = 2
	state = INIT
	
	clientInfo = {}
	active_clients = False

	true_frameNumber = 0
	current_frameNumber = 0
	last_frame = 0

	def read_bootstrapper(self):

		data = requests.get('http://0.0.0.0:8080')
		print(f"\ndata: {data.text}")
		lines = data.text.split('\n')
		for line in lines[:-1]:
			linha = line.split('-')
			self.topologia[linha[0]] = linha[1].split(',')

		print(self.topologia)
		return self.topologia

	def flood(self,SERVER_ADDRESS, video_file):
		vizinhos = self.topologia[SERVER_ADDRESS]

		payload = {"source": SERVER_ADDRESS,
				  "jumps": 0,
				  "timestamp": int(time.time()*1000)
				  }
		packet = bytearray([])
		packet.append(0)
		pack_data = msgpack.packb(payload, use_bin_type=True)

		size = len(pack_data) + 3
		packet += size.to_bytes(2,'big')
		packet += pack_data

		for vizinho in vizinhos:
			self.clientInfo[vizinho] = {}
			self.clientInfo[vizinho]['videoStream'] = VideoStream(self.VIDEO_PATH + video_file)
			self.clientInfo[vizinho]['state'] = self.READY
			self.clientInfo[vizinho]['active_clients'] = False
			self.clientInfo[vizinho]['address'] = vizinho
			self.clientInfo[vizinho]['rtpPort'] = self.RTP_PORT
			s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
			try:
				s.connect((vizinho, self.NODE_PORT))
				s.send(packet)
			except:
				print(f"Connection to {vizinho} failed\n")		

	def processRtspRequest(self, data, addr):
		"""Process RTSP request sent from the client."""
		# Get the request type
		# request = data.split(' ')
		# requestType = request[0]
		requestType = int.from_bytes([data[0]],'big')
		size = int.from_bytes(data[1:3],'big')

		# Get the RTSP sequence number
		# seq = request[1]
		print(self.clientInfo)
		# Process SETUP request
		if requestType == self.SETUP:
			if self.clientInfo[addr]['state'] == self.INIT:

				# Update state
				print("processing SETUP\n")

				try:
					payload = msgpack.unpackb(data[3:size], raw=False)
					print(payload)
					self.clientInfo[addr]['videoStream'] = VideoStream(self.VIDEO_PATH + self.video_file)
					self.clientInfo[addr]['state'] = self.READY
				except:
					print("Error in payload or in video Stream\n")
				
				# Generate a randomized RTSP session ID
				#self.clientInfo['session'] = randint(100000, 999999)
				
				# Send RTSP reply
				# self.replyRtsp(self.OK_200, seq)
				
				# Get the RTP/UDP port from the last line
				self.clientInfo[addr]['rtpPort'] = self.RTP_PORT

		
		# Process PLAY request 		
		elif requestType == self.PLAY:
			if self.clientInfo[addr]['state'] == self.READY:
				self.clientInfo[addr]['active_clients'] = True
				print("processing PLAY\n")
				self.clientInfo[addr]['state'] = self.PLAYING
				
				# Create a new socket for RTP/UDP
				self.clientInfo[addr]["rtpSocket"] = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
				
				#self.replyRtsp(self.OK_200, seq)
				
				# Create a new thread and start sending RTP packets
				# self.clientInfo['event'] = threading.Event()
				#self.clientInfo[addr]['worker']= threading.Thread(target=self.worker_list[addr].sendRtp) 
				#self.clientInfo[addr]['worker'].start()
		
		# Process TEARDOWN request
		elif requestType == self.TEARDOWN:

			self.clientInfo[addr]['active_clients'] = False
			print("processing TEARDOWN\n")

			# self.clientInfo['event'].set()
			#self.replyRtsp(self.OK_200, seq)
			
			# Close the RTP socket
			# self.clientInfo['rtpSocket'].close()

	def main(self):
		
		try:
			SERVER_PORT = int(sys.argv[1])
			video_file = sys.argv[2]
			SERVER_ADDRESS = sys.argv[3] # aqui é o ip

		except:
			print("[Usage: Server.py Server_port Video_file SERVER_ADDRESS]\n")
		
		self.read_bootstrapper()
		self.flood(SERVER_ADDRESS, video_file)

		for vizinho in self.topologia[SERVER_ADDRESS]:
			ServerWorker(self.clientInfo[vizinho],video_file).run()

		rtspSocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		rtspSocket.bind(('', SERVER_PORT))
		rtspSocket.listen(5)

		while True:
			sock,(addr,_) = rtspSocket.accept()
			data = sock.recv(1500)
			self.processRtspRequest(data,addr)


		# Receive client info (address,port) through RTSP/TCP session
		#while True:
		#	
		#	clientInfo = {}
		#	clientInfo['rtspSocket'] = rtspSocket.accept()
		#	ServerWorker(clientInfo,video_file).run()		

if __name__ == "__main__":
	(Server()).main()


