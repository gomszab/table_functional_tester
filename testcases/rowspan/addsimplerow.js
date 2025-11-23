test(async () => {
	const form = queryFormById('jsform')
	assertNotUndefined(form)


	const [_table, originalRows ]= queryTableByTbodyId('jstbody', true);
    

	setInputValueByid(form, 'elso','Test item 1')
	setInputValueByid(form, 'masodik','Test item 2')
	setInputValueByid(form, 'harmadik','Test item 3')



	await triggerSubmit(form)
	
	const [_, newRows ]= queryTableByTbodyId('jstbody', true);
	assertEquals(originalRows+1, newRows, "Nem fuzott hozza sort");
    const lastRow = getLastRowFromTableByTbodyId('jstbody');
    assertEquals("Test item 1", lastRow[0].content);
    assertEquals("Test item 2", lastRow[1].content);
    assertEquals("Test item 3", lastRow[2].content);
    for(let i=0; i< lastRow.length; i++){
        assertEquals(1, lastRow[i].colspan);
        assertEquals(1, lastRow[i].rowspan);
    }
    return true;
})