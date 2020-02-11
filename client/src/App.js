import React from 'react';
import Get from './components/Get'
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>
          This is test UI to interact with Rust server
        </h1>
        <Get />
      </header>
    </div>
  );
}

export default App;
