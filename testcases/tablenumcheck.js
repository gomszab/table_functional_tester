test(() => {
	const tables = queryAllFromBody('table')
	return assertEquals(2, tables.length);
})