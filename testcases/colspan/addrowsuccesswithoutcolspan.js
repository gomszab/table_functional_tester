 test(async ()=> {

    const form = queryFormById(colform)
	assertNotUndefined(form)

    const [_table, originalRows ]= queryTableByTbodyId(coltablebody, true);

    setInputValueByid(form, 'elso','Test item 1')
    setInputValueByid(form, 'masodik', 'Test item 2');
	setInputValueByid(form, 'harmadik','Test item 3')
    setInputValueByid(form, 'negyedik','Test item 4')

    await triggerSubmit(form)

    const [__table, newRows ]= queryTableByTbodyId(coltablebody, true);

    assertEquals(originalRows+1, newRows, "Nem adott hozza sort");
    const lastRow = getLastRowFromTableByTbodyId(coltablebody);
    assertEquals(4, lastRow.length);
    assertEquals("Test item 1", lastRow[0].content, "A hozzáadott táblázathoz hozzáadott elem 1. oszlopának tartalma nem jó");
    assertEquals("Test item 2", lastRow[1].content, "A hozzáadott táblázathoz hozzáadott elem 2. oszlopának tartalma nem jó");
    assertEquals("Test item 3", lastRow[2].content, "A hozzáadott táblázathoz hozzáadott elem 3. oszlopának tartalma nem jó");
    assertEquals("Test item 4", lastRow[3].content, "A hozzáadott táblázathoz hozzáadott elem 4. oszlopának tartalma nem jó");
    for(let i=0; i< lastRow.length; i++){
        assertEquals(1, lastRow[2].colspan, `A hozzáadott táblázathoz hozzáadott elem ${i+1}. oszlopának colspan értéke nem jó`);
        assertEquals(1, lastRow[2].rowspan, `A hozzáadott táblázathoz hozzáadott elem ${i+1}. oszlopának rowspan értéke nem jó`);
    }
    return true;
 })