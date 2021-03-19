export const file = async () => {
  const linreg = await import('wasml-linreg');

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.readCSV(file);
    console.log('Table before pop', table.headers);
    const sex = table.pop('sex');
    console.log('Popped table', sex.data);
    console.log('Table after pop', table.headers);
  } catch (err) {
    console.error(err);
  }
};

export const run = async () => {
  try {
    const linreg = await import('wasml-linreg');
    const col = new linreg.Column('hello', ['hello', 1, 'world']);
    console.log(col.data, col.data);
  } catch (err) {
    console.error(err);
  }
};
