# Initialising the framework
An app will be initialised in a few steps. One first initialisation call will
set up anything the framework needs to and create an OpenGL context. A
framework handle will then be returned. This handle is used to mount components
to the root.

Adding children to a node constitutes 'dirtying' that node, so nodes will
normally be created in reverse order (i.e. child up) for components containing
'slots' (this will be quite common for top level nodes).


