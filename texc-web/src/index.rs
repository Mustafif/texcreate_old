pub const INDEX: &str = r#"<!DOCTYPE html>
<html>
<title>TexCreate</title>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
<link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Montserrat">
<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">
<style>
body, h1,h2,h3,h4,h5,h6 {font-family: "Montserrat", sans-serif}
.w3-row-padding img {margin-bottom: 12px}
/* Set the width of the sidebar to 120px */
.w3-sidebar {width: 120px;background: #222;}
/* Add a left margin to the "page content" that matches the width of the sidebar (120px) */
#main {margin-left: 120px}
/* Remove margins from "page content" on small screens */
@media only screen and (max-width: 600px) {#main {margin-left: 0}}
</style>
<body class="w3-black">
<!-- Page Content -->
<div class="w3-padding-large" id="main">
    <!-- Header/Home -->
    <header class="w3-container w3-padding-32 w3-center w3-black" id="home">
        <h1 class="w3-jumbo"><span class="w3-hide-small">TexCreate</span></h1>
    <!-- <img src="Banner.png" alt="boy" class="w3-image" width="*" height="0.25*"> -->
    </header>

    <!-- About Section -->
    <div class="w3-content w3-justify w3-text-grey w3-padding-64" id="about">
        <h2 class="w3-text-light-grey">BUILD A LaTeX PROJECT</h2>
        <hr style="width:200px" class="w3-opacity">
            <div class="w3-container w3-center">
<form method="post" action="/">
    <label for="author">Author</label><br>
    <input  class="w3-teal w3-button" type="text" name="author" id="author" placeholder="Author"><br>
    <label for="title">Title</label><br>
    <input class="w3-teal w3-button" type="text" name="title" id="title" placeholder="Title"><br>
    <label for="date">Date</label><br>
    <input class="w3-teal w3-button" type="text" name="date" id="date" placeholder="Date"><br>
    <label for="project_name">Project Name</label><br>
    <input class="w3-teal w3-button" type="text" name="project_name" id="project_name" placeholder="Project Name"><br>
    <label for="template">Template</label><br>
    <select class="w3-teal w3-button" type="text" name="template" id="template" placeholder="Template">
    <option value="Basic">Basic</option>
    <option value="Math">Math</option>
    <option value="Code">Code</option>
    <option value="Theatre">Theatre</option>
    <option value="Novel">Novel</option>
    <option value="Beamer">Beamer</option>
    <option value="Lachaise">Lachaise</option>
    <option value="Lachaise-Mod"> Lachaise Modified</option>
    <option value="Dictionary">Dictionary</option>
    <option value="News">News</option>
    </select>
    <br>
    <label for="paper_size">Paper Size</label><br>
    <input class="w3-teal w3-button" type="text" name="paper_size" id="paper_size" placeholder="Paper Size"><br>
    <label for="font_size">Font Size</label><br>
    <input class="w3-teal w3-button" type="text" name="font_size" id="font_size" placeholder="Font Size"><br>
    <label for="document_class">Document Class</label><br>
    <input class="w3-teal w3-button" type="text" name="document_class" id="document_class" placeholder="Document Class"><br>
    <br>
    <input class="w3-teal w3-button" type="text" name="only_files" id="only_files" placeholder="Only Files (true/false)"><br>
    <br>
    <input class="w3-btn w3-teal" type="submit" value="Submit">
</form>
</div>

    </div>

    <!-- Footer -->
    <footer class="w3-content w3-padding-64 w3-text-grey w3-xlarge">
        <p class="w3-medium">Built by  <a href="https://github.com/MKProj/" target="_blank" class="w3-hover-text-green">MKProjects</a></p>
        <!-- End footer -->
    </footer>
    <!-- END PAGE CONTENT -->
</div>
</body>
</html>

"#;
