// this was coded by my dad lol...
const express = require("express");
const bodyParser = require("body-parser");
const cors = require("cors"); // do we actually need cors here?
app.use(cors());

const app = express();
const port = 3000;

app.use(bodyParser.json());

let clickCount = 0; // reset click count to 0

// Wait for the DOM to be ready
document.addEventListener('DOMContentLoaded', function () {
  console.log('Page loaded!');
  // Get the button element by its ID
  var nyabtn = document.getElementById('nyabtn');

  // Add a click event listener to the button
  nyabtn.addEventListener('click', function () {
      // This function will be called when the button is clicked
      console.log('Button clicked!');

      // You can add your custom logic here
      // For example, make an AJAX request, update the UI, etc.
      var numElement = document.getElementById('counterNum');
      numElement.innerHTML = clickCount++;
  });
});


// Serve static files (such as your HTML and script) from the root directory
app.use(express.static("public_html"));

app.listen(port, () => {
  console.log(`Server is running on port ${port}`); // just say port 3000?
});
