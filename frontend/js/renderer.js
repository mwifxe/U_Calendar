const Renderer = {
    renderCalendario(data) {
        let html = '';

        data.periodos.forEach(periodo => {
            html += `
                <div class="periodo">
                    <div class="periodo-header">
                        <div class="periodo-number">${periodo.numero}</div>
                        <div class="periodo-info">
                            <h2>${periodo.nombre}</h2>
                            <p>${periodo.info}</p>
                        </div>
                    </div>
            `;

            periodo.semanas.forEach(semana => {
                const clases = [];
                if (semana.es_parcial) clases.push('parcial');
                if (semana.es_vacaciones) clases.push('vacaciones');

                html += `
                    <div class="semana ${clases.join(' ')}" data-semana-id="${semana.id}">
                        <div class="semana-header">
                            <div class="semana-title">${this.escapeHtml(semana.titulo)}</div>
                            <div class="fecha-badge">${this.escapeHtml(semana.fecha)}</div>
                            ${!semana.es_parcial && !semana.es_vacaciones
                    ? '<button class="btn-add-actividad">+</button>'
                    : ''}
                        </div>
                `;

                if (semana.es_parcial && semana.parciales) {
                    html += '<div class="parciales-grid">';
                    semana.parciales.forEach(parcial => {
                        html += `<div class="parcial-item">${this.escapeHtml(parcial)}</div>`;
                    });
                    html += '</div>';
                } else if (semana.es_vacaciones) {
                    html += '<div class="sin-actividades">Esta semana no cuenta en el período académico</div>';
                } else {
                    html += '<div class="actividades">';
                    semana.actividades.forEach(actividad => {
                        html += this.renderActividad(actividad);
                    });
                    html += '</div>';
                    html += '<div class="sin-actividades">Sin actividades evaluadas</div>';

                    if (semana.nota_especial) {
                        html += `<div class="nota-especial">${this.escapeHtml(semana.nota_especial)}</div>`;
                    }
                }

                html += '</div>'; // .semana
            });

            html += '</div>'; // .periodo
        });

        return html;
    },

    renderActividad(actividad) {
        const clase = actividad.clase || '';
        const hasNotes = actividad.tiene_notas;

        let metaHtml = '';
        if (actividad.fecha) {
            metaHtml += `<div class="fecha-actividad">${this.escapeHtml(actividad.fecha)}</div>`;
        }
        if (actividad.porcentaje) {
            metaHtml += `<div class="porcentaje">${this.escapeHtml(actividad.porcentaje)}</div>`;
        }

        return `
            <div class="actividad ${clase} ${hasNotes ? 'has-notes' : ''}"
                 data-id="${actividad.id}"
                 data-materia="${this.escapeAttr(actividad.materia)}"
                 data-titulo="${this.escapeAttr(actividad.titulo)}"
                 data-fecha="${this.escapeAttr(actividad.fecha || '')}"
                 data-porcentaje="${this.escapeAttr(actividad.porcentaje || '')}"
                 data-descripcion="${this.escapeAttr(actividad.descripcion || '')}">
                <div class="actividad-header">
                    <div class="actividad-left">
                        <div class="materia-nombre">${this.escapeHtml(actividad.materia)}</div>
                        <div class="actividad-titulo">${this.escapeHtml(actividad.titulo)}</div>
                        <div class="actividad-meta">${metaHtml}</div>
                    </div>
                    <div class="actividad-actions">
                        <button class="btn-edit-actividad" data-actividad-id="${actividad.id}">Editar</button>
                        <button class="btn-delete-mini" data-actividad-id="${actividad.id}">✕</button>
                    </div>
                </div>
            </div>
        `;
    },

    escapeHtml(str) {
        if (!str) return '';
        const div = document.createElement('div');
        div.textContent = str;
        return div.innerHTML;
    },

    escapeAttr(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#39;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;');
    },
};