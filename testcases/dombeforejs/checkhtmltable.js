test(()=> { 
	const tables = queryAllFromBody('table:not([data-js-generated="true"])')
	return assertEquals(1, tables.length, "nincs az oldalon html táblázat");
})