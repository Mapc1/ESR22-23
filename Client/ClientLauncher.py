import sys
from tkinter import Tk
from Client import Client

if __name__ == "__main__":
	try:
		nodeAddr = sys.argv[1]
		rtpsPort = sys.argv[2]
		rtpPort = sys.argv[3]
	except:
		print("[Usage: ClientLauncher.py node_addr RTPS_Port RTP_port]")
		exit(1)

	if not nodeAddr or not rtpsPort or not rtpPort:
		print("[Usage: ClientLauncher.py node_addr RTPS_Port RTP_port]")
		exit(1)

	root = Tk()

	# Create a new client
	app = Client(root, nodeAddr, rtpsPort, rtpPort)
	app.master.title("RTPClient")
	root.mainloop()
