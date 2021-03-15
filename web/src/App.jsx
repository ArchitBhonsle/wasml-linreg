import { useEffect } from 'react';

import './App.css';

function App() {
  useEffect(() => {
    (async () => {
      const linreg = await import('wasml-linreg');
      linreg.greet();
    })();
  }, []);

  return (
    <div className='app'>
      <h1>WASML - LinReg</h1>
      <p>Linear Regression in the browser powered by WebAssembly</p>
    </div>
  );
}

export default App;
