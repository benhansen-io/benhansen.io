+++
date = 2022-10-04
title = "Linux Directory Permissions"
+++
<style>
  table.styled {
    border-collapse: collapse;
  }
  .styled td, .styled th {
    border: thin solid black;
    text-align: center;
    padding: 0.3rem;
  }
  .styled thead tr {
    background-color: #F0F0F0;
    text-align: left;
  }
  .read {
    color: #904000;
  }
  .write {
    color: #8B0000;
  }
  .execute {
    color: #008000;
  }
  .term {
    font-family: Menlo, Consolas, Monaco, Liberation Mono, Lucida Console, monospace;
    border: thin solid black;
    padding: 0.5rem;
    margin: 1rem;
  }
  .term.command {
    background-color: #F0F0F0;
  }
  .term .output {
    white-space: pre;
    display: none;
  }
</style>

<span class="read">Read</span>, <span class="write">write</span> and <span class="execute">execute</span> permissions are used for directories as well as files on Linux but their meaning for directories is not as straight forward.

For directories the permission bit meanings are:

<table class="styled">
  <thead>
    <tr>
      <th>
        Permission
      </th>
      <th>
        Symbol
      </th>
      <th>
        <a href="https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation">Value</a>
      </th>
      <th>
        Description for directories
      </th>
    </tr>
  </thead>
  <tr>
    <td>
      <span class="read">Read</span>
    </td>
    <td>
      r
    </td>
    <td>
      4
    </td>
    <td>
      Allows listing the names of entries in the directory (e.g. using "ls") but not entry metadata or contents.
    </td>
  </tr>
  <tr>
    <td>
      <span class="write">Write</span>
    </td>
    <td>
      w
    </td>
    <td>
      2
    </td>
    <td>
      Allows adding, removing or moving entries (but not their contents).
    </td>
  </tr>
  <tr>
    <td>
      <span class="execute">Execute</span>
    </td>
    <td>
      x
    </td>
    <td>
      1
    </td>
    <td>
      Allows accessing and modifying the entry contents and metadata. Also needed to add, delete or remove entries. Also called "Search" for directories<sup><a href="#ref1">1</a></sup>.
    </td>
  </tr>
</table>

## Permissions needed for an action

Or listed inversely:

<table class="styled">
  <thead>
    <tr>
      <th>
        Action
      </th>
      <th>
        Required Permission(s)
      </th>
    </tr>
  </thead>
  <tr>
    <td>
      Listing the entries of the directory
    </td>
    <td>
      <span class="read">Read</span>
    </td>
  </tr>
  <tr>
    <td>
      "cd" into the directory
    </td>
    <td>
      <span class="read">Read</span>
    </td>
  </tr>
  <tr>
    <td>
      Adding or removing entries from the directory
    </td>
    <td>
      <span class="write">Write</span> and <span class="execute">Execute</span> (see also "Restricted Deletion" below)
    </td>
  </tr>
  <tr>
    <td>
      Accessing the directory entries' contents or entry metadata
    </td>
    <td>
      <span class="execute">Execute</span>
    </td>
  </tr>
  <tr>
    <td>
      Modifying the directory entries' contents or entry metadata
    </td>
    <td>
      <span class="execute">Execute</span>
    </td>
  </tr>
</table>


## Playground

See how the following commands are handled when executed on a directory named `testdir` that starts with a single file named `A.txt` when the executing user has the following permissions:

<form id="permissions_form">
  <span class="read">
    <input type="checkbox" id="read" name="read">
    <label for="read">Read</label>
  </span>
  <span class="write">
    <input type="checkbox" id="write" name="write">
    <label for="write">Write</label>
  </span>
  <span class="execute">
    <input type="checkbox" id="execute" name="execute">
    <label for="execute">Execute</label>
  </span>
</form>
<div class="term command">$ chmod u=<span id="mode"></span> testdir</div>

{{ linux_directory_permissions_commands() }}

<script>
  const r = document.getElementById("read");
  const w = document.getElementById("write");
  const x = document.getElementById("execute");
  const modeLabel = document.getElementById("mode");
  const form = document.getElementById("permissions_form");
  
  function updateOutputs() {
    let modeString = (r.checked ? "r" : "")
      + (w.checked ? "w" : "")
      + (x.checked ? "x" : "");
    modeLabel.innerHTML = modeString

    if (modeString == "") {
      modeString = "EMPTY";
    }
    const outputs = document.querySelectorAll(".term .output");
    for (output of outputs) {
      output.style.display = output.classList.contains(modeString) ? "block" : "none";
    }
  }

  updateOutputs();
  form.addEventListener("input", (evt) => {
    updateOutputs();
  });
</script>

## Examples explained

* A directory where your user has <span class="execute">execute</span> but not <span class="read">read</span> permissions would allow you to edit or modify files that you can not list with "ls" if you already know their path.
* A directory where your user has <span class="write">write</span> and <span class="execute">execute</span> but not <span class="read">read</span> would allow you to add new files and modify existing files but not see them.
* A directory with only <span class="write">write</span> permissions is the same as a directory with no permissions<sup><a href="#ref2">2</a></sup>.
* Most of the time you will always want to set <span class="execute">execute</span> if you are also setting either <span class="read">read</span> or <span class="write">write</span>.

## More advanced permissions

* Restricted Deletion (also called Sticky Bit) - For directories only lets the file and directory owner move or delete a file inside the directory. 
* Set Group Identity (setgid) - For directories causes new files to be created with the same group as the directory rather than the group of the processes creating the file.
* Set User Identity (setuid) - Ignored by Linux and most unix systems.

## References

<span id="ref1">1</span>: <a href="https://man7.org/linux/man-pages/man1/chmod.1.html">man 1 chmod</a> is the main source of this page's information.

<span id="ref2">2</span>: See [this SO question](https://unix.stackexchange.com/a/149291/45680). I like the OPs analogy of "execute" allowing for the mapping to inode.
