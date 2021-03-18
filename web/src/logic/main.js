export const file = async () => {
  const linreg = await import('wasml-linreg');

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.tableFromCSV(file);
    console.log('Table before pop', table.headers);
    const sex = table.pop('sex');
    console.log('Popped table', sex.data);
    console.log('Table after pop', table.headers);
  } catch (err) {
    console.error(err);
  }

  try {
    const wnum = new linreg.Warray([10, 1.0, 300, -1], 'num');
    console.log(wnum.data, wnum.datatype);
    const numcol = new linreg.Column('hello', wnum);
    console.log(numcol.header, numcol.warray);
    console.log(numcol.data);
  } catch (err) {
    console.error(err);
  }
};
