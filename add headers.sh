#!/bin/bash

HEADER="// hardware-diagnostic - Ferramenta de diagnóstico de hardware
// Copyright (C) $(date +%Y)  Seu Nome
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
"

for file in src/*.rs; do
    if [ -f "$file" ]; then
        # Verifica se já tem cabeçalho
        if ! head -n 20 "$file" | grep -q "GNU General Public"; then
            # Adiciona cabeçalho
            echo "$HEADER" > temp_file
            cat "$file" >> temp_file
            mv temp_file "$file"
            echo "Adicionado cabeçalho GPL em: $file"
        fi
    fi
done