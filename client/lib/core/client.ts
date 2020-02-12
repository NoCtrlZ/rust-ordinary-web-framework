import net from 'net'

const client = new net.Socket();
client.setEncoding('utf8');

export default class Client {
	port :number
	basePath :string | undefined

	constructor(port? :number) {
		this.port = port ? port : 443
	}

	setBaseURL(basePath :string) {
		this.basePath = basePath
	}

	get(path :string) {
		console.log(this.port, String(this.basePath))
		client.connect(this.port, String(this.basePath), () => {
			console.log("send")
			client.write('GET / HTTP/1.1\r\nHost: localhost:5000\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\ntest');
		})

		client.on('data', (data :any) => {
			console.log('Received: ' + data);
			client.destroy();
		});

		client.on('close', () => {
			console.log('Connection closed');
		});
	}
}