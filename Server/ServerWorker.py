from random import randint
from VideoStream import VideoStream
from RtpPacket import RtpPacket
import sys, traceback, threading, socket, time, msgpack

class ServerWorker:
	
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

	OK_200 = 0
	FILE_NOT_FOUND_404 = 1
	CON_ERR_500 = 2
	
	clientInfo = {}
	active_clients = False

	true_frameNumber = 0
	current_frameNumber = 0
	last_frame = 0
	
	def __init__(self, clientInfo,video_file):
		self.clientInfo = clientInfo
		self.video_file = video_file
		
	def run(self):
		threading.Thread(target=self.sendRtp).start()
	
	def recvRtspRequest(self):
		"""Receive RTSP request from the client."""		

		connSocket = self.clientInfo['rtspSocket'][0]
		while True:            
			data = connSocket.recv(1500)
			if data:
				#print("Data received:\n" + data.decode("utf-8"))
				self.processRtspRequest(data)
	
	def processRtspRequest(self, data):
		"""Process RTSP request sent from the client."""
		# Get the request type
		# request = data.split(' ')
		# requestType = request[0]
		requestType = int.from_bytes(data[0],'big')
		size = int.from_bytes(data[1:5],'big')

		# Get the RTSP sequence number
		# seq = request[1]
		
		# Process SETUP request
		if requestType == self.SETUP:
			if self.clientInfo['state'] == self.INIT:

				# Update state
				print("processing SETUP\n")

				try:
					payload = msgpack.unpackb(data[5:size+5], raw=False)
					print(payload)
					self.clientInfo['videoStream'] = VideoStream(self.VIDEO_PATH + self.video_file)
					self.clientInfo['state'] = self.READY
				except:
					print("Error in payload or in video Stream\n")
				
				# Generate a randomized RTSP session ID
				self.clientInfo['session'] = randint(100000, 999999)
				
				# Send RTSP reply
				# self.replyRtsp(self.OK_200, seq)
				
				# Get the RTP/UDP port from the last line
				self.clientInfo['rtpPort'] = self.RTP_PORT

		
		# Process PLAY request 		
		elif requestType == self.PLAY:
			if self.clientInfo['state'] == self.READY:
				self.active_clients = True
				print("processing PLAY\n")
				self.clientInfo['state'] = self.PLAYING
				
				# Create a new socket for RTP/UDP
				self.clientInfo["rtpSocket"] = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
				
				#self.replyRtsp(self.OK_200, seq)
				
				# Create a new thread and start sending RTP packets
				# self.clientInfo['event'] = threading.Event()
				self.clientInfo['worker']= threading.Thread(target=self.sendRtp) 
				self.clientInfo['worker'].start()
		
		# Process PAUSE request
		#elif requestType == self.PAUSE:
		#	if self.clientInfo['state'] == self.PLAYING:
		#
		#		self.active_clients = False
		#		print("processing PAUSE\n")
		#		self.clientInfo['state'] = self.READY
		#		
		#		# self.clientInfo['event'].set()
		#	
		#		#self.replyRtsp(self.OK_200, seq)
		
		# Process TEARDOWN request
		elif requestType == self.TEARDOWN:

			self.active_clients = False
			print("processing TEARDOWN\n")

			# self.clientInfo['event'].set()
			# self.replyRtsp(self.OK_200, seq)
			
			# Close the RTP socket
			# self.clientInfo['rtpSocket'].close()
			
	def sendRtp(self):
		"""Send RTP packets over UDP."""
		print("Sending UDPs")
		while True:
			
			data = self.clientInfo['videoStream'].nextFrame()
			time.sleep(0.06)
			
			# Stop sending if request is PAUSE or TEARDOWN
			# Ele só deve enviar quando tem clientes, mas continua a "reproduzir" o vídeo
			if not self.clientInfo['active_clients']:
				# print("\nNot sending\n")			
				pass

			else:	
				if data: 
					self.current_frameNumber = self.clientInfo['videoStream'].frameNbr()
					self.true_frameNumber = self.current_frameNumber + self.last_frame
					
					try:
						address = self.clientInfo['rtspSocket'][1][0]
						port = int(self.clientInfo['rtpPort'])
						self.clientInfo['rtpSocket'].sendto(self.makeRtp(data, self.true_frameNumber),(address,port))
					except:
						print("Connection Error")
						print('-'*60)
						traceback.print_exc(file=sys.stdout)
						print('-'*60)
				else:
					print("De novo")
					self.clientInfo['videoStream'] = VideoStream(self.VIDEO_PATH + self.video_file)
					self.last_frame = self.true_frameNumber

	def sendPings(self):
		
		while True:
				
			address = self.clientInfo['rtspSocket'][1][0]
			port = int(self.clientInfo['pingPort'])
			msg = bytes("Web é o futuro",'UTF-8')
			print(f"{address} {port}")
			try:
				self.clientInfo['pingSocket'].sendto(msg,(address,port))

			except:
				print("\nPing failed\n")
			
			time.sleep(3)


	def makeRtp(self, payload, frameNbr):
		"""RTP-packetize the video data."""
		version = 2
		padding = 0
		extension = 0
		cc = 0
		marker = 0
		pt = 26 # MJPEG type
		seqnum = frameNbr
		ssrc = 0 
		
		rtpPacket = RtpPacket()
		
		rtpPacket.encode(version, padding, extension, cc, seqnum, marker, pt, ssrc, payload)
		
		return rtpPacket.getPacket()
		
	def replyRtsp(self, code, seq):
		"""Send RTSP reply to the client."""
		if code == self.OK_200:
			print("200 OK")
			reply = 'RTSP/1.0 200 OK\nCSeq: ' + seq + '\nSession: ' + str(self.clientInfo['session'])
			connSocket = self.clientInfo['rtspSocket'][0]
			connSocket.send(reply.encode())
		
		# Error messages
		elif code == self.FILE_NOT_FOUND_404:
			print("404 NOT FOUND")
		elif code == self.CON_ERR_500:
			print("500 CONNECTION ERROR")

# [type u8, size  u16] -> Data[size] port:1234
# 0 flooding "0 128 "
# 1 request packet (play)
# 2 ACK
# 3 Teardown

# Server:Flooding, bootstrapper, adaptar os requests
# Cliente: