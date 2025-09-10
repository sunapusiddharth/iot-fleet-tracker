import {
  DataGrid,
  GridToolbarContainer,
  GridToolbarColumnsButton,
  GridToolbarFilterButton,
  GridToolbarDensitySelector,
  GridToolbarExport,
  type GridColDef,
  type GridRowsProp,
  type GridPaginationModel,
  type GridFilterModel,
  type GridSortModel,
  type GridRowSelectionModel,
  type DataGridProps,
} from '@mui/x-data-grid';

function CustomToolbar() {
  return (
    <GridToolbarContainer>
      <GridToolbarColumnsButton />
      <GridToolbarFilterButton />
      <GridToolbarDensitySelector />
      <GridToolbarExport />
    </GridToolbarContainer>
  );
}

interface DataTableProps extends Partial<DataGridProps> {
  columns: GridColDef[];
  rows: GridRowsProp;
  loading?: boolean;
  pagination?: true|undefined;
  onPaginationModelChange?: (model: GridPaginationModel) => void;
  rowCount?: number;
  filterModel?: GridFilterModel;
  onFilterModelChange?: (model: GridFilterModel) => void;
  sortingModel?: GridSortModel;
  onSortingModelChange?: (model: GridSortModel) => void;
  checkboxSelection?: boolean;
  onSelectionModelChange?: (model: GridRowSelectionModel) => void;
  selectionModel?: GridRowSelectionModel;
  toolbar?: React.ElementType;
}

const DataTable: React.FC<DataTableProps> = ({
  columns,
  rows,
  loading = false,
  pagination = true,
  onPaginationModelChange,
  rowCount,
  filterModel,
  onFilterModelChange,
  sortingModel,
  onSortingModelChange,
  checkboxSelection = false,
  onSelectionModelChange,
  selectionModel,
  toolbar,
  ...props
}) => {
  return (
    <div style={{ height: 600, width: '100%' }}>
      <DataGrid
        rows={rows}
        columns={columns}
        loading={loading}
        pagination={pagination}
        paginationMode="server"
        onPaginationModelChange={onPaginationModelChange}
        rowCount={rowCount}
        pageSizeOptions={[10, 25, 50, 100]}
        filterMode="server"
        filterModel={filterModel}
        onFilterModelChange={onFilterModelChange}
        sortingMode="server"
        sortingModel={sortingModel}
        onSortingModelChange={onSortingModelChange}
        checkboxSelection={checkboxSelection}
        onSelectionModelChange={onSelectionModelChange}
        selectionModel={selectionModel}
        slots={{
          toolbar: toolbar || CustomToolbar,
        }}
        slotProps={{
          toolbar: {
            showQuickFilter: true,
            quickFilterProps: { debounceMs: 500 },
          },
        }}
        {...props}
      />
    </div>
  );
};

export default DataTable;
