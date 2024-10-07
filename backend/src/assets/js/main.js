import { beforeSubmit, handleSelectAndRadioElements, handleFileElements, handleAutocompleteElements, handleCheckboxElements } from './form.js';
import { init } from './quill.js';

function sortTable(columnName, order) {
    console.log(`Sorting by ${columnName} in ${order} order`);
}

function handleColumnClick(event) {
    const th = event.target.closest('th[data-sort]');
    if (!th) return;

    const columnName = th.dataset.sort;
    const currentOrder = th.dataset.order || 'none';

    let newOrder;
    if (currentOrder === 'asc') {
        newOrder = 'desc';
    } else if (currentOrder === 'desc') {
        newOrder = 'none';
    } else {
        newOrder = 'asc';
    }

    updateUrlParameter(columnName, newOrder);

    if (newOrder !== 'none') {
        sortTable(columnName, newOrder);
    }
    th.dataset.order = newOrder;
}

function updateUrlParameter(key, value) {
    const urlParams = new URLSearchParams(window.location.search);
    const params = [];

    if (urlParams.has('order')) {
        const orders = urlParams.get('order').split(',');
        let found = false;
        orders.forEach(order => {
            const [col, dir] = order.split('_');
            if (col === key) {
                found = true;
                if (value !== 'none') {
                    params.push(`${key}_${value}`);
                }
            } else {
                params.push(order);
            }
        });
        if (!found && value !== 'none') {
            params.push(`${key}_${value}`);
        }
    } else if (value !== 'none') {
        params.push(`${key}_${value}`);
    }

    if (params.length > 0) {
        urlParams.set('order', params.join(','));
    } else {
        urlParams.delete('order');
    }

    window.history.replaceState({}, '', `${window.location.pathname}?${urlParams.toString()}`);
}

document.addEventListener('DOMContentLoaded', function () {
    handleCheckboxElements();
    handleSelectAndRadioElements();
    handleFileElements();
    handleAutocompleteElements();
    init();
    beforeSubmit();

    document.querySelectorAll('th[data-sort]').forEach(th => {
        th.addEventListener('click', handleColumnClick);
    });
});
