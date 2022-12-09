import sys, socket
from ServerWorker import ServerWorker

class Server:

	def read_bootstrapper():
		pass
	
	def main(self):
		try:
			SERVER_PORT = int(sys.argv[1])
			video_file = sys.argv[2]

		except:
			print("[Usage: Server.py Server_port Video_file]\n")
		


		rtspSocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		rtspSocket.bind(('', SERVER_PORT))
		rtspSocket.listen(5)


		# Receive client info (address,port) through RTSP/TCP session
		while True:
			clientInfo = {}
			clientInfo['rtspSocket'] = rtspSocket.accept()
			ServerWorker(clientInfo,video_file).run()		

if __name__ == "__main__":
	(Server()).main()


