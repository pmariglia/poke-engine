Installation
============

Poke Engine can be installed with :code:`pip`, but the engine must be compiled from source.
This means **you must have Rust / Cargo installed on your system to pip install**

.. code-block:: bash

    pip install poke-engine

Without additional arguments the engine is compiled for generation 4 when installing.

To compile the engine for a different generation, the :code:`--config-settings` flag can be used.
For example, to compile for generation 5:

.. code-block:: bash

    pip install poke-engine \
    --config-settings="build-args=--features poke-engine/gen5 --no-default-features"

If adding to a :code:`requirements.txt` file, the following line can be used:

.. code-block:: bash

    poke-engine --config-settings="build-args=--features poke-engine/gen5 --no-default-features"
