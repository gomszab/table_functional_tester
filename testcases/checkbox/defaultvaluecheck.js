test(() => {
    const checkbox = queryFromBody(`input[id="${checkboxId}"]`);
    return assertEquals(checkboxdefault, checkbox.checked, "Checkbox alapértelmezett állapota nem megfelelő");
})