const Modals = {
    currentActivityId: null,
    currentSemanaId: null,
    editingActivityId: null,

    async openNotas(actividadEl) {
        const id = actividadEl.dataset.id;
        this.currentActivityId = id;

        document.getElementById('modalNotasMateria').textContent = actividadEl.dataset.materia;
        document.getElementById('modalNotasTitulo').textContent = actividadEl.dataset.titulo;

        const modalInfo = document.getElementById('modalNotasInfo');
        modalInfo.innerHTML = '';
        if (actividadEl.dataset.fecha) {
            modalInfo.innerHTML += `<div class="fecha-actividad">${actividadEl.dataset.fecha}</div>`;
        }
        if (actividadEl.dataset.porcentaje) {
            modalInfo.innerHTML += `<div class="porcentaje">${actividadEl.dataset.porcentaje}</div>`;
        }

        try {
            const nota = await API.getNota(id);
            document.getElementById('notasTextarea').value = nota.contenido || '';
        } catch (e) {
            console.error('Error loading nota:', e);
            document.getElementById('notasTextarea').value = '';
        }

        this._show('modalNotasOverlay', 'modalNotas');
    },

    closeNotas() {
        this._hide('modalNotas', 'modalNotasOverlay');
    },

    async saveNotas() {
        const contenido = document.getElementById('notasTextarea').value;
        const btn = document.getElementById('btnSaveNotas');

        try {
            btn.disabled = true;
            btn.textContent = 'Guardando...';
            await API.saveNota(this.currentActivityId, contenido);

            // Update visual indicator
            const actividadEl = document.querySelector(`[data-id="${this.currentActivityId}"]`);
            if (actividadEl) {
                if (contenido.trim()) {
                    actividadEl.classList.add('has-notes');
                } else {
                    actividadEl.classList.remove('has-notes');
                }
            }

            this.closeNotas();
        } catch (e) {
            console.error('Error saving nota:', e);
            alert('Error al guardar la nota');
        } finally {
            btn.disabled = false;
            btn.textContent = 'Guardar';
        }
    },

    openActividad(semanaId, actividadData = null) {
        this.currentSemanaId = semanaId;
        this.editingActivityId = actividadData ? actividadData.id : null;

        const form = document.getElementById('formActividad');

        if (actividadData) {
            document.getElementById('modalActividadTitulo').textContent = 'Editar Actividad';
            document.getElementById('inputMateria').value = actividadData.materia;
            document.getElementById('inputNombre').value = actividadData.titulo;
            document.getElementById('inputFecha').value = actividadData.fecha || '';
            document.getElementById('inputPorcentaje').value = actividadData.porcentaje || '';
            document.getElementById('inputDescripcion').value = actividadData.descripcion || '';
        } else {
            document.getElementById('modalActividadTitulo').textContent = 'Nueva Actividad';
            form.reset();
        }

        this._show('modalActividadOverlay', 'modalActividad');
    },

    closeActividad() {
        this._hide('modalActividad', 'modalActividadOverlay');
    },

    async saveActividad() {
        const materia = document.getElementById('inputMateria').value;
        const titulo = document.getElementById('inputNombre').value;
        const fecha = document.getElementById('inputFecha').value || null;
        const porcentaje = document.getElementById('inputPorcentaje').value || null;
        const descripcion = document.getElementById('inputDescripcion').value || null;

        if (!materia || !titulo) {
            alert('Por favor completa la materia y el nombre de la actividad');
            return;
        }

        const data = { materia, titulo, fecha, porcentaje, descripcion };
        const btn = document.getElementById('btnSaveActividad');

        try {
            btn.disabled = true;
            btn.textContent = 'Guardando...';

            if (this.editingActivityId) {
                await API.updateActividad(this.editingActivityId, data);
            } else {
                await API.createActividad(this.currentSemanaId, data);
            }

            this.closeActividad();
            await App.loadCalendario();
        } catch (e) {
            console.error('Error saving actividad:', e);
            alert('Error al guardar la actividad');
        } finally {
            btn.disabled = false;
            btn.textContent = 'Guardar Actividad';
        }
    },

    async deleteActividad(actividadId) {
        if (!confirm('¿Estás seguro de que quieres eliminar esta actividad?')) return;

        try {
            await API.deleteActividad(actividadId);
            await App.loadCalendario();
        } catch (e) {
            console.error('Error deleting actividad:', e);
            alert('Error al eliminar la actividad');
        }
    },

    _show(overlayId, modalId) {
        document.body.classList.add('modal-open');
        document.getElementById(overlayId).classList.add('active');
        setTimeout(() => {
            document.getElementById(modalId).classList.add('active');
        }, 10);
    },

    _hide(modalId, overlayId) {
        document.getElementById(modalId).classList.remove('active');
        setTimeout(() => {
            document.getElementById(overlayId).classList.remove('active');
            document.body.classList.remove('modal-open');
        }, 300);
    },
};