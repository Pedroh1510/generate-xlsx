import { pipeline } from 'node:stream/promises';
import XLSX from 'exceljs';
import { createPool, createConnection } from 'mariadb';
import { setTimeout } from 'node:timers/promises';
import { Transform, Writable } from 'node:stream';

const pool = createPool({
	host: 'localhost',
	user: 'root',
	password: 'mauFJcuf5dhRMQrjj',
	database: 'base',
	port: 3306,
	timeout: 1000,
	connectTimeout: 1000,
	initializationTimeout: 1000
});
async function handleDb(query) {
	const conn = await pool.getConnection();
	return conn.query(query);
}
async function handleStreamDb(query) {
	const conn = await pool.getConnection();
	// const conn = await createConnection({
	// 	allowPublicKeyRetrieval: true,
	// 	host: 'localhost',
	// 	user: 'root',
	// 	password: 'mauFJcuf5dhRMQrjj',
	// 	database: 'base',
	// 	timeout: 1000,
	// 	connectTimeout: 1000,
	// 	initializationTimeout: 1000
	// });

	// return conn.queryStream(query);
	return conn.queryStream(query).on('close', () => conn.release());
}
async function main() {
	console.log('Iniciando');
	const oneRow = await handleDb('SELECT * FROM User2 LIMIT 1');
	const headers = Object.keys(oneRow[0]).map((item) => ({
		header: item,
		key: item
	}));
	const dataStream = await handleStreamDb('SELECT * FROM User2');
	const xlsx = new XlsxWriter(headers, dataStream);
	await pipeline(dataStream, xlsx);
	console.log('Finalizado');
	process.exit(0);
}
main();
class XlsxWriter extends Writable {
	constructor(columns, stream) {
		super({ objectMode: true });
		this.xlsx = new XLSX.stream.xlsx.WorkbookWriter({
			filename: 'data.xlsx',
			useSharedStrings: false,
			useStyles: true
		});
		this.ws = this.xlsx.addWorksheet();
		this.ws.columns = columns;
		this.counter = 0;
		this.stream = stream;
	}
	async _write(chunk, enc, cb) {
		this.stream.pause();
		this.counter++;
		this.ws.addRow(chunk).commit();
		await setTimeout(1);
		if (this.counter % 1000 === 0) {
			await setTimeout(100);
			console.log(this.counter);
		}
		this.stream.resume();
		cb();
	}
	async _final(cb) {
		await this.xlsx.commit();
		cb(null);
	}
}
