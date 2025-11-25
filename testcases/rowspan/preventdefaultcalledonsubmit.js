 test(async ()=> {

    const form = queryFormById(rowform)
	assertNotUndefined(form)

    await triggerSubmit(form)

    return assertEquals(true, window.__submitPrevented);
 })