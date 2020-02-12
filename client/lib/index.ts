import Client from './core/client'

const sendRequest = () => {
    const client = new Client(5000)
    client.setBaseURL('127.0.0.1')
    client.get("/")
}

sendRequest()
