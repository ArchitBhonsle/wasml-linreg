import { file, run } from './logic/main';

import './App.css';
import { useEffect } from 'react';

function App() {
  useEffect(() => run(), []);

  return (
    <div className='app'>
      <h1>WASML - LinReg</h1>
      <p>Linear Regression in the browser powered by WebAssembly</p>
      <button onClick={() => file()}>File</button>
    </div>
  );
}

export default App;
