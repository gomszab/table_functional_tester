test(async () => {
    const form = queryFormById(rowform)
    assertNotUndefined(form)


    const [_table, originalRows] = queryTableByTbodyId(rowtablebody, true);


    setInputValueByid(form, 'elso', 'Test item')
    setInputValueByid(form, 'harmadik', 'Test item1')


    await triggerSubmit(form)

    const [_, newRows] = queryTableByTbodyId(rowtablebody, true);
    const errorMessageMasodik = getErrorFieldContentByInputId(form, 'masodik');
    assertNotEmptyString(errorMessageMasodik);
    assertEquals(originalRows, newRows, "Sort fűzött hozzá a táblázathoz a validáció ellenére");
    setInputValueByid(form, 'masodik', 'test item 2');
    await triggerSubmit(form)
    const elemList = ['elso', 'masodik', 'harmadik'];
    elemList.map(elem =>
        getErrorFieldContentByInputId(form, elem)
    ).forEach((element, index) => {
        assertEmptyString(element, `${index + 1}. beviteli mező erorüzenete nem tünt el`);
    });
    const [__, newRowsAgain] = queryTableByTbodyId(rowtablebody, true);
    return assertNotEquals(originalRows + 1, newRowsAgain, "Validációs hibák javítása után nem jelent meg új sor");
})