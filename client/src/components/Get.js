import React from 'react';
import axios from 'axios';

export default class Get extends React.Component {
    constructor(props) {
        super(props)
        this.state = { x: 1, y: 2, data: ''}
    }
    sendGet(e) {
        axios.get(`https://localhost:5000`)
        .then(res => {
            const data = res.data
            this.setState({data: data})
        })
    }
    render() {
        return (
        <div>
            <p>Send Get Request For Rust Server</p>
            <p>Param x : {this.state.x}, y : {this.state.y}</p>
            <button onClick={() => this.sendGet()}>Send Get Request</button>
        </div>)
    }
}
