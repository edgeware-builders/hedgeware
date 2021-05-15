export const loadJSON = () => {
	const file = require('fs').readFileSync('./dump/accounts.json');
	const json = JSON.parse(file);
	return json;
};

export const migrateRefcount = (json: any) => {
	return json.map((elt: any) => {
		return {
			...elt,
			value: {
	            nonce: 0,
	            consumers: 0,
	            providers: 1,
	            sufficients: 0,
	            data: {
	            	...elt.value.data,
	            }
			}
		}
	});
}
