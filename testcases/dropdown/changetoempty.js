test(async () => {
    const select = queryFromBody('select[id="tableselector"]')
    assertNotUndefined(select, "nincs select elem a megadott id-val");
    select.value = '';

    await triggerEvent(select, 'change');

    const divList = queryAllFromBody('body > div');
    for (const div of divList) {
        assertEquals(true, div.classList.contains('hide'), `A ${div.id} azonosítójú elem nincs elrejtve`);
    }
    return true;
}, hascheckbox)