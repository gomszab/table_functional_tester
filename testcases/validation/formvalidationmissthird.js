test(async () => {
	const form = queryFormById('htmlform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);


	setInputValueByid(form, 'masodik','Test item')
	setInputValueByid(form, 'elso','Test item1')


	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('htmltbody', true);
    const errorMessageThird = getErrorFieldContentByInputId(form, 'harmadik');
    assertNotEmptyString(errorMessageThird);
	return assertEquals(originalRows, newRows, "Nem fuzott hozza sort");
})