const test = async (callback, ignore = false) => {
	try {
		if (!ignore) {
			const result = await callback();
			if (result) {
				return JSON.stringify({
					"result": true,
					"ignored": false
				})
			}
		} else {
			return JSON.stringify({
				"result": true,
				"ignored": ignore
			})
		}

	} catch (e) {
		return JSON.stringify({
			"result": false,
			"ignored": ignore,
			"message": e.message
		})
	}
}

(() => {
	const originalAppendChild = Node.prototype.appendChild;
	Node.prototype.appendChild = function (child) {
		if (child instanceof Element) {
			child.setAttribute('data-js-generated', 'true');
		}
		return originalAppendChild.call(this, child);
	};

	const originalPreventDefault = Event.prototype.preventDefault;

	Event.prototype.preventDefault = function () {
		if (this.type === 'submit') {
			window.__submitPrevented = true;
		}
		return originalPreventDefault.apply(this, arguments);
	};
})();

const triggerSubmit = (form) => {
	const promise = promisifyEvent(form, 'submit', (e) => e.preventDefault());
	form.dispatchEvent(new Event('submit', { bubbles: true, cancelable: true }));
	return promise;
}

const promisifyEvent = (htmlElement, eventType, extraCallback = (_e) => { }) => {
	return new Promise((resolve) => {
		htmlElement.addEventListener(eventType, (e) => {
			extraCallback(e);
			resolve();
		})
	})
}

const triggerEvent = (htmlElement, eventType) => {
	const promise = promisifyEvent(htmlElement, eventType);
	htmlElement.dispatchEvent(new Event(eventType, { bubbles: true, cancelable: true }));
	return promise;
}