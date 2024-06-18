document.addEventListener('DOMContentLoaded', function() {
    beforeSubmit();
    handleSelectAndRadioElements();
    autocomplete();

    const Block = Quill.import('blots/block');
    const Container = Quill.import('blots/container');
    const Break = Quill.import('blots/break');
    const TextBlot = Quill.import('blots/text');
    const Cursor = Quill.import('blots/cursor');

    class CodeBlockContainer extends Container {
        static create(value) {
            const domNode = super.create(value) ;
            domNode.setAttribute('spellcheck', 'false');
            domNode.setAttribute('class', 'prism-code ql-code-block-container ');
            return domNode;
        }
    }
    
    class CodeBlock extends Block {
        static TAB = '  ';
        static register() {
          Quill.register(CodeBlockContainer);
        }
    }

    CodeBlockContainer.blotName = 'code-block-container';
    CodeBlockContainer.tagName = 'pre';
    CodeBlockContainer.allowedChildren = [CodeBlock];

    CodeBlock.blotName = 'code-block';
    CodeBlock.className = 'ql-code-block';
    CodeBlock.tagName = 'DIV';
    CodeBlock.allowedChildren = [TextBlot, Break, Cursor];
    CodeBlock.requiredContainer = CodeBlockContainer;
      
    Quill.register(CodeBlock);

    var editorElements = document.querySelectorAll('.editor');
    editorElements.forEach(function(editorElement) {
        var editorId = editorElement.id;
        var textarea = document.querySelector(`textarea[data-editor-id="${editorId}"]`);
        textarea.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
            var slug = createSlug(titleElement.textContent);
            titleElement.id = slug;
        });
        var initialContent = textarea ? textarea.value : '';
        var quill = new Quill(editorElement, {
            theme: 'snow',
            modules: {
                syntax: true,
                toolbar: [
                    ['bold', 'italic', 'underline', 'strike'],
                    ['blockquote', 'code-block'],
                    [{ 'list': 'ordered'}, { 'list': 'bullet' }],
                    [{ 'script': 'sub'}, { 'script': 'super' }],
                    [{ 'indent': '-1'}, { 'indent': '+1' }],
                    [{ 'direction': 'rtl' }],
                    [{ 'size': ['small', false, 'large', 'huge'] }],
                    [{ 'header': [1, 2, 3, 4, 5, 6, false] }],
                    [{ 'color': [] }, { 'background': [] }],
                    [{ 'font': [] }],
                    [{ 'align': [] }],
                    ['clean'],
                ],
            }
        });
        if (initialContent !== '') {
            quill.root.innerHTML = initialContent;
        }
        quill.on('text-change', function(delta, oldDelta, source) {
            var root = quill.root
            root.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
                var slug = createSlug(titleElement.textContent);
                titleElement.id = slug;
            });
            if (source === 'user') {
                textarea.value = quill.root.innerHTML;
            }
        });
    });
});

function autocomplete() {
    const autocompleteFields = document.querySelectorAll('.autocomplete');
    const form = document.querySelector('form');
    
    const autocompleteSelectedItems = document.querySelectorAll('.autocomplete-selected-items');
    autocompleteSelectedItems.forEach(function(element) {
        element.innerHTML = '';
    });
    const itemsSelected = [];
    autocompleteFields.forEach(field => {
        field.addEventListener('input', function() {
            const id = this.getAttribute('id');
            const multiSelectSuggestionsList = form.querySelector(`#${id}List`);
            
            multiSelectSuggestionsList.innerHTML = '';
            multiSelectSuggestionsList.style.display = "block";
            if (this.value.length >= 3) {
                multiSelectSuggestionsList.style.display = "block";
                fetchData(`${this.getAttribute('data-url')}/${this.value}`, function(data) {
                    Object.keys(data).forEach(function(key) {
                        const item = document.createElement('li');
                        item.textContent = data[key][field.getAttribute('data-label')];
                        item.setAttribute('id', data[key][field.getAttribute('data-id')]);
                        item.classList.add('p-2', 'hover:bg-gray-100', 'cursor-pointer');
                        multiSelectSuggestionsList.appendChild(item);
                    });
                });
            } else {
                multiSelectSuggestionsList.style.display = "none";
            }
        });
    });
    const multiSelectSuggestionsList = form.querySelector(`.autocomplete-list`);
    multiSelectSuggestionsList.addEventListener('click', function(event) {
        if (event.target.tagName.toLowerCase()!== 'li') return;
        multiSelectSuggestionsList.style.display = "none";
        const selectedId = event.target.getAttribute('id');
        if (itemsSelected.includes(selectedId)) {
            return;
        }
        itemsSelected.push(selectedId);
        const parentElementDataId = event.target.parentElement.getAttribute('data-id');
        const checkbox = document.createElement('input');
        checkbox.id = `${selectedId}-autocomplete-selected`;
        checkbox.setAttribute("type", "checkbox");
        checkbox.setAttribute("style", "display:none");
        checkbox.setAttribute("name", `${parentElementDataId}`);
        checkbox.setAttribute("value", selectedId);
        checkbox.setAttribute("checked", 'checked');
        const selectedItemContainer = form.querySelector(`#${parentElementDataId}Selected`);
        const removeBtn = document.createElement('span');
        removeBtn.className = `
            remove-from-list
            inline-flex 
            items-center
            gap-x-1.5 
            rounded-md 
            bg-indigo-600 
            px-3 
            py-2 
            text-sm 
            font-semibold 
            text-white 
            shadow-sm 
            hover:bg-indigo-500 
            focus-visible:outline 
            focus-visible:outline-2 
            focus-visible:outline-offset-2 
            focus-visible:outline-indigo-600
            `;
        removeBtn.textContent = event.target.textContent;
        removeBtn.appendChild(crossSvg());
        selectedItemContainer.appendChild(removeBtn);
        selectedItemContainer.appendChild(checkbox);
        removeBtn.addEventListener('click', function() {
            selectedItemContainer.removeChild(checkbox);
            selectedItemContainer.removeChild(removeBtn);
            itemsSelected = itemsSelected.filter(element => element!== selectedId);
        });
    });
}

function crossSvg() {
    let svgIcon = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svgIcon.setAttribute("viewBox", "0 0 24 24");
    svgIcon.setAttribute("width", "15");
    svgIcon.setAttribute("height", "15");
    svgIcon.setAttribute("className", "-mr-0.5 h-5 w-5 cursor-pointer");
    let path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    path.setAttribute("d", "M22.245,4.015c0.313,0.313,0.313,0.826,0,1.139l-6.276,6.27c-0.313,0.312-0.313,0.826,0,1.14l6.273,6.272  c0.313,0.313,0.313,0.826,0,1.14l-2.285,2.277c-0.314,0.312-0.828,0.312-1.142,0l-6.271-6.271c-0.313-0.313-0.828-0.313-1.141,0  l-6.276,6.267c-0.313,0.313-0.828,0.313-1.141,0l-2.282-2.28c-0.313-0.313-0.313-0.826,0-1.14l6.278-6.269  c0.313-0.312,0.313-0.826,0-1.14L1.709,5.147c-0.314-0.313-0.314-0.827,0-1.14l2.284-2.278C4.308,1.417,4.821,1.417,5.135,1.73  L11.405,8c0.314,0.314,0.828,0.314,1.141,0.001l6.276-6.267c0.312-0.312,0.826-0.312,1.141,0L22.245,4.015z");
    svgIcon.appendChild(path);
    return svgIcon;
}

function beforeSubmit() {
    const form = document.querySelector('form');
    const datetimeFields = form.querySelectorAll('input[type="datetime-local"]');
    datetimeFields.forEach(function(field) {
        var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
        field.value = datetime_field.value;
    });
    form.addEventListener('submit', function(event) {
        event.preventDefault();
        form.querySelectorAll('h1, h2, h3, h4, h5, h6').forEach(function(titleElement) {
            var slug = createSlug(titleElement.textContent);
            titleElement.setAttribute('id', slug);
        });
        datetimeFields.forEach(function(field) {
            var datetime_field = document.querySelector(`input[data-datetime="${field.id}"]`);
            let date = new Date(field.value);
            datetime_field.value = date.toISOString().slice(0, 19);
        });
        form.submit();
    });
}

function handleSelectAndRadioElements() {
    var selectElements = document.querySelectorAll('.select');
    var radioElements = document.querySelectorAll('.radio');
    selectElements.forEach(function(selectElement) {
        fetchData(selectElement.getAttribute('data-url'), function(data) {
            populateSelectOptions(selectElement, data);
        });
    });
    radioElements.forEach(function(radioElement) {
        fetchData(radioElement.getAttribute('data-url'), function(data) {
            populateRadioOptions(radioElement, data);
        });
    });
}

function fetchData(url, processData) {
    fetch(url, {
        method: 'GET', 
        headers: {
            'Content-Type': 'application/json' 
        }
    })
    .then(function(response) {
        if (!response.ok) {
            throw new Error('Fetch options error');
        }
        return response.json();
    })
    .then(processData)
    .catch(function(error) {
        console.error('Fetch options error:', error);
    });
}

function populateSelectOptions(selectElement, data) {
    selectElement.innerHTML = '';
    Object.keys(data).forEach(function(key) {
        var option = document.createElement('option');
        option.value = data[key][selectElement.getAttribute('data-id')];
        option.textContent = data[key][selectElement.getAttribute('data-label')];
        selectElement.appendChild(option);
    });
    var selectedValue = selectElement.getAttribute('data-selected');
    if (selectedValue) {
        selectElement.value = selectedValue;
    }
}

function populateRadioOptions(radioElement, data) {
    radioElement.innerHTML = '';
    Object.keys(data).forEach(function(key) {
        var name = radioElement.getAttribute('data-name');
        var value = data[key][radioElement.getAttribute('data-id')];
        var labelText = data[key][radioElement.getAttribute('data-label')];
        var selected = radioElement.getAttribute('data-selected');
        var radioHtml = createRadioInput(name, value, labelText, selected);
        radioElement.innerHTML += radioHtml;
    });
}

function createRadioInput(name, value, labelText, selected) {
    return `
        <div class="sm:col-span-4">
            <input id="${name}" name="${name}" value="${value}" ${selected == value ? 'checked' : ''} type="radio"
                class="h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-600">
            <label for="${name}"
                class="block text-sm font-medium leading-6 text-gray-900">${labelText}</label>
        </div>
    `;
}

function createSlug(text) {
    return text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '');
}
