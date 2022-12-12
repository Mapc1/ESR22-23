import sys
from tkinter import Tk
from Client import Client

if __name__ == "__main__":
	try:
		nodeAddr = sys.argv[1]
		serverPort = sys.argv[2]
		rtpPort = sys.argv[3]
		pingPort = sys.argv[4]
	except:
		print("[Usage: ClientLauncher.py node_addr server_port RTP_port ping_port]")
		exit(1)

	root = Tk()

	# Create a new client
	app = Client(root, nodeAddr, serverPort, rtpPort, pingPort)
	app.master.title("RTPClient")
	root.mainloop()
