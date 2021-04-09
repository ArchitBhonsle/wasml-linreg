import { time } from './utils';

export const file = async () => {
  const linreg = await import('wasml-linreg');
  const { Transform, Watrix, LinearRegression } = linreg;

  const [fileHandle] = await window.showOpenFilePicker();
  const file = await fileHandle.getFile();

  try {
    const table = await linreg.readCSV(file);
    console.log(table.data);

    const transform = new Transform(table);
    const transformedTable = table.applyTransform(transform);
    console.log(transformedTable.data);

    const matrix = Watrix.newFromTable(transformedTable);
    console.log('Entire dataset', matrix.dims);

    matrix.shuffle_mut();

    const [rows, cols] = [matrix.nrows, matrix.ncols];
    const trainQty = Math.floor(rows * 0.7);
    const train = matrix.row_slice(0, trainQty);
    const test = matrix.row_slice(trainQty, matrix.nrows);

    const [x_train, y_train] = [
      train.col_slice(0, cols - 1),
      train.col_slice(cols - 1, cols),
    ];
    console.log('Training split', x_train.dims, y_train.dims);
    const [x_test, y_test] = [
      test.col_slice(0, cols - 1),
      test.col_slice(cols - 1, cols),
    ];
    console.log('Testing split', x_test.dims, y_test.dims);

    const lr = new LinearRegression(0.01, 150);
    const [timeToTrain] = time(() => lr.fit(x_train, y_train));
    console.log(`Time taken to train: ${timeToTrain / 1000} secs`);

    const [timeToTest, result] = time(() => lr.predict(x_test, y_test));
    console.log(`Time taken to train: ${timeToTest / 1000} secs`);
    console.log(result);
  } catch (err) {
    console.error(err);
  }
};

export const run = async () => {
  try {
  } catch (err) {
    console.error(err);
  }
};
