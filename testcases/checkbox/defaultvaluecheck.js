test(() => {
    const checkbox = queryFromBody(`input[id="${checkboxId}"]`);
    return assertEquals(checkboxdefault, checkbox.checked, "Checkbox állapota nem megfelelő");
})