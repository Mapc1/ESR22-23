from VideoStream import VideoStream
from RtpPacket import RtpPacket
import sys, traceback, threading, time

class ServerWorker:
	
	true_frameNumber = 0
	current_frameNumber = 0
	last_frame = 0
	
	def __init__(self, clientInfo,video_file):
		self.clientInfo = clientInfo
		self.video_file = video_file


	def run(self):
		threading.Thread(target=self.sendRtp).start()
	
			
	def sendRtp(self):
		"""Send RTP packets over UDP."""
		print("Sending UDPs")
		while True:
			True
			data = self.clientInfo['videoStream'].nextFrame()
			time.sleep(0.06)
			
			# Stop sending if request is PAUSE or TEARDOWN
			# Ele só deve enviar quando tem clientes, mas continua a "reproduzir" o vídeo
			if not self.clientInfo['active_clients']:
				#print("\nNot sending\n")			
				pass

			else:	
				if data: 
					self.current_frameNumber = self.clientInfo['videoStream'].frameNbr()
					self.true_frameNumber = self.current_frameNumber + self.last_frame
					
					try:
						address = self.clientInfo['address']
						port = int(self.clientInfo['rtpPort'])
						print(f'sending {len(data)} seq {self.true_frameNumber}...')
						self.clientInfo['rtpSocket'].sendto(self.makeRtp(data, self.true_frameNumber),(address,port))
					except:
						print("Connection Error")
						print('-'*60)
						traceback.print_exc(file=sys.stdout)
						print('-'*60)
				else:
					print("De novo")
					self.clientInfo['videoStream'] = VideoStream(self.video_file)
					self.last_frame = self.true_frameNumber


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
