import React from 'react';
import './App.css';

function App(props) {
  const sendGet = e => {
    console.log(e)
  }
  return (
    <div className="App">
      <header className="App-header">
        <p>
          This is test UI to interact with Rust server
        </p>
        <button onClick={() => sendGet(props.arg)}>Send Get Request</button>
      </header>
    </div>
  );
}

export default App;
