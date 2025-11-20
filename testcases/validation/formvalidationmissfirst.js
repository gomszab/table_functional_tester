test(async () => {
	const form = queryFormById('htmlform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);


	setInputValueByid(form, 'masodik','Test item')
	setInputValueByid(form, 'harmadik','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('htmltbody', true);
    const errorMessageElso = getErrorFieldContentByInputId(form, 'elso');
    assertNotEmptyString(errorMessageElso);
	return assertEquals(originalRows, newRows, "Nem fuzott hozza sort");
})