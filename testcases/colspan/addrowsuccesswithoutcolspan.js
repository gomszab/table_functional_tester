 test(async ()=> {

    const form = queryFormById('htmlform')
	assertNotUndefined(form)

    const [_table, originalRows ]= queryTableByTbodyId('htmltbody', true);

    setInputValueByid(form, 'elso','Test item 1')
    setInputValueByid(form, 'masodik', 'Test item 2');
	setInputValueByid(form, 'harmadik','Test item 3')
    setInputValueByid(form, 'negyedik','Test item 4')

    await triggerSubmit(form)

    const [__table, newRows ]= queryTableByTbodyId('htmltbody', true);

    assertEquals(originalRows+1, newRows, "Nem adott hozza sort");
    const lastRow = getLastRowFromTableByTbodyId('htmltbody');
    assertEquals(4, lastRow.length);
    assertEquals("Test item 1", lastRow[0].content);
    assertEquals("Test item 2", lastRow[1].content);
    assertEquals("Test item 3", lastRow[2].content);
    assertEquals("Test item 4", lastRow[3].content);
    for(let i=0; i< lastRow.length; i++){
        assertEquals(1, lastRow[2].colspan);
        assertEquals(1, lastRow[2].rowspan);
    }
    return true;
 })