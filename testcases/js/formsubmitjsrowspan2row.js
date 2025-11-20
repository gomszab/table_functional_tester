test(async () => {
	const form = queryFormById('jsform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('jstbody', true);


	setInputValueByid(form, 'elso','Test item')
	setInputValueByid(form, 'masodik','Test item1')
	setInputValueByid(form, 'harmadik','Test item1')
	setInputValueByid(form, 'negyedik','Test item1')
	setInputValueByid(form, 'otodik','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('jstbody', true);
	return assertEquals(originalRows+2, newRows, "Nem fuzott hozza sort");
})