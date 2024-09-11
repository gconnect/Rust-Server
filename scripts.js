function expandDirectory(path) {
  fetch(`/?path=${encodeURIComponent(path)}`)
      .then(response => response.text())
      .then(html => {
          const element = document.querySelector(`[onclick="expandDirectory('${path}')"]`).parentElement;
          element.innerHTML += html;
      })
      .catch(error => console.error('Error:', error));
}

function readFile(path) {
  fetch(`/?path=${encodeURIComponent(path)}`)
      .then(response => response.text())
      .then(content => {
          const displayElement = document.getElementById('file-display');
          displayElement.innerHTML = content;
      })
      .catch(error => console.error('Error:', error));
}
