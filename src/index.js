import config from './config';
import Client from './Client';

const client = new Client();

if (config.mode === `GET`) client.getInput();
else if (config.mode === `SOLVE`) client.submitAnswer(fs.readFileSync(path.resolve(__dirname, `../staging/output.txt`), `utf-8`));
