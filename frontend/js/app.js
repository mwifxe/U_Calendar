const App = {
    calendarioData: null,

    _boundClickHandler: null,

    async loadCalendario() {
        const container = document.getElementById('container');
        const loading = document.getElementById('loading');
        const errorState = document.getElementById('error-state');

        try {
            this.calendarioData = await API.getCalendario();

            container.innerHTML = Renderer.renderCalendario(this.calendarioData);
            container.style.display = '';
            loading.style.display = 'none';
            errorState.style.display = 'none';

            this.attachListeners();
        } catch (e) {
            console.error('Error loading calendario:', e);
            loading.style.display = 'none';
            errorState.style.display = '';
        }
    },

    attachListeners() {
        const container = document.getElementById('container');

        // Remove old listener to prevent duplicates on reload
        if (this._boundClickHandler) {
            container.removeEventListener('click', this._boundClickHandler);
        }
        this._boundClickHandler = (e) => this.handleContainerClick(e);
        container.addEventListener('click', this._boundClickHandler);
    },

    handleContainerClick(e) {
        const target = e.target;

        if (target.classList.contains('btn-add-actividad')) {
            e.stopPropagation();
            const semanaId = target.closest('.semana').dataset.semanaId;
            Modals.openActividad(semanaId);
            return;
        }

        if (target.classList.contains('btn-edit-actividad')) {
            e.stopPropagation();
            const actividadId = target.dataset.actividadId;
            const actividadEl = target.closest('.actividad');
            const semanaId = target.closest('.semana').dataset.semanaId;

            Modals.openActividad(semanaId, {
                id: actividadId,
                materia: actividadEl.dataset.materia,
                titulo: actividadEl.dataset.titulo,
                fecha: actividadEl.dataset.fecha,
                porcentaje: actividadEl.dataset.porcentaje,
                descripcion: actividadEl.dataset.descripcion,
            });
            return;
        }

        if (target.classList.contains('btn-delete-mini')) {
            e.stopPropagation();
            const actividadId = target.dataset.actividadId;
            Modals.deleteActividad(actividadId);
            return;
        }

        const actividadEl = target.closest('.actividad');
        if (actividadEl) {
            Modals.openNotas(actividadEl);
        }
    },

    init() {
        this.loadCalendario();

        document.getElementById('btnSaveNotas').addEventListener('click', (e) => {
            e.stopPropagation();
            Modals.saveNotas();
        });

        document.getElementById('btnCloseNotas').addEventListener('click', (e) => {
            e.stopPropagation();
            Modals.closeNotas();
        });

        document.getElementById('modalNotasOverlay').addEventListener('click', (e) => {
            if (e.target.id === 'modalNotasOverlay') Modals.closeNotas();
        });

        document.getElementById('btnSaveActividad').addEventListener('click', (e) => {
            e.stopPropagation();
            Modals.saveActividad();
        });

        document.getElementById('btnCloseActividad').addEventListener('click', (e) => {
            e.stopPropagation();
            Modals.closeActividad();
        });

        document.getElementById('modalActividadOverlay').addEventListener('click', (e) => {
            if (e.target.id === 'modalActividadOverlay') Modals.closeActividad();
        });

        document.getElementById('modalNotas').addEventListener('click', (e) => {
            e.stopPropagation();
        });

        document.getElementById('modalActividad').addEventListener('click', (e) => {
            e.stopPropagation();
        });
    },
};

document.addEventListener('DOMContentLoaded', () => App.init());