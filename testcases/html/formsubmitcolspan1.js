test(async () => {
	const form = queryFormById('htmlform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);


	setInputValueByid(form, 'elso','Test item')
	setInputValueByid(form, 'masodik','Test item1')
	setInputValueByid(form, 'harmadik','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('htmltbody', true);
	return assertEquals(originalRows+1, newRows, "Nem fuzott hozza sort");
})