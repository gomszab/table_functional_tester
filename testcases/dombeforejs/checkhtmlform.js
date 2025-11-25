test(()=> { 
	const tables = queryAllFromBody('form:not([data-js-generated="true"])')
	return assertEquals(1, tables.length);
})