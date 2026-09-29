# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'Firkin'
copyright = '2026, Kyle Duke'
author = 'Kyle Duke'
release = '0.3.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.doctest',
    'sphinx.ext.autodoc',
    'sphinx.ext.autosummary',
    'sphinx.ext.apidoc',
]

templates_path = ['_templates']
exclude_patterns = []



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'alabaster'
html_static_path = ['_static']

# Load unit definitions

from os import path
import re
import pandas as pd

DIR_PATH = path.dirname(path.dirname(path.dirname(path.realpath(__file__))))

def load_unit_definitions():
    aliases = pd.read_csv(path.join(DIR_PATH, "unit_definitions", "aliases.csv"))
    base_units = pd.read_csv(path.join(DIR_PATH, "unit_definitions", "base_units.csv"))
    derived_units = pd.read_csv(path.join(DIR_PATH, "unit_definitions", "derived_units.csv"))
    log_units = pd.read_csv(path.join(DIR_PATH, "unit_definitions", "log_units.csv"))

    derived_units = derived_units.drop(["offset", "scale", "derivation"], axis=1)
    log_units = log_units.drop(["scale_factor", "scale_factor_per"], axis=1)

    base_units["aliases"] = ""
    derived_units["aliases"] = ""

    for i, aliasrow in aliases.iterrows():
        for j, burow in base_units.iterrows():
            if burow["name"] == aliasrow["name"]:
                base_units.loc[j, "aliases"] += "\n- " + aliasrow["alias"]
        for j, durow in derived_units.iterrows():
            if durow["name"] == aliasrow["name"]:
                derived_units.loc[j, "aliases"] += "\n- " + aliasrow["alias"]

    rename_mapper = {
        "name": "Name",
        "abbr": "Abbreviation or Symbol",
        "python_var_name": "Python Variable Name",
        "aliases": "Aliases",
    }

    for i, row in derived_units.iterrows():
        m = re.match(r"\\u{(.*)}", row["abbr"])
        if m is None:
            continue
        derived_units.loc[i, "abbr"] = row["abbr"].replace(m.group(0), f"\\u{m.group(1):0>4}").encode().decode("unicode-escape")

    base_units = base_units.rename(rename_mapper, axis=1)
    derived_units = derived_units.rename(rename_mapper, axis=1)
    log_units = log_units.rename(rename_mapper, axis=1)

    base_units.to_csv(path.join(DIR_PATH, "docs", "source", "api", "firkin.units", "base_units.csv"), index=False)
    derived_units.to_csv(path.join(DIR_PATH, "docs", "source", "api", "firkin.units", "derived_units.csv"), index=False)
    log_units.to_csv(path.join(DIR_PATH, "docs", "source", "api", "firkin.units", "log_units.csv"), index=False)

load_unit_definitions()